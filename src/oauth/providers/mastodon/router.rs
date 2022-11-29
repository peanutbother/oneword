use super::MastodonProvider;
use crate::util::oauth_safe;
use axum::{
    extract::Query,
    response::{IntoResponse, Redirect},
    Extension,
};
use entity::{
    sea_orm::{ActiveModelTrait, ModelTrait, PaginatorTrait},
    DatabaseConnection,
};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use tokio::sync::OnceCell;

// #[asxum::debug_handler]
pub async fn request_token(
    Extension(db): Extension<OnceCell<DatabaseConnection>>,
    Query(query): Query<RequestCodeQuery>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let db = db.get().expect("failed to retrieve database connection");
    let RequestCodeQuery {
        guild: guild_id,
        instance,
        force_insecure,
    } = query;
    let force_insecure = force_insecure.unwrap_or_default();

    if entity::guild::Entity::find_by_id(guild_id)
        .count(db)
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed to query database".to_owned(),
            )
        })?
        > 0
    {
        MastodonProvider::request_auth(guild_id, instance, force_insecure)
            .await
            // .map(|v| (StatusCode::OK, v.0.to_string()).into_response())
            .map(|v| Redirect::temporary(v.0.as_str()))
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    e.to_string(), /* .into() */
                )
            })
    } else {
        Err((StatusCode::BAD_REQUEST, "guild not registered".to_owned()))
    }
}

pub async fn oauth_callback(
    Extension(db): Extension<OnceCell<DatabaseConnection>>,
    Query(query): Query<ResponseCodeQuery>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let db = db.get().expect("failed to retrieve database");
    let ResponseCodeQuery { code, state } = query;
    let ResponseState { guild, instance } = serde_json::from_str(&state).map_err(|e| {
        log::error!("{e}");
        (
            StatusCode::BAD_REQUEST,
            "failed to validate state, please try again!".to_owned(),
        )
    })?;

    if MastodonProvider::verify_code(code.clone(), guild).await {
        let (url, config) =
            MastodonProvider::request_token(instance.clone(), code).map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    e.to_string(), /* .into() */
                )
            })?;
        let old = oauth_safe(db, guild, "mastodon".to_owned())
            .await
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "failed to update token database".to_owned(),
                )
            })?;

        let ResponseToken {
            access_token: token,
        } = reqwest::Client::new()
            .post(url)
            .json(&config)
            .send()
            .await
            .map_err(|e| {
                log::error!("{e:#?}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to request token from instance, please try again later.".to_owned(),
                )
            })?
            .json()
            .await
            .map_err(|e| {
                log::error!("{e:?}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to deserialize resposne from instance, please try again later."
                        .to_owned(),
                )
            })?;

        old.delete(db).await.map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed to delete old token from database".to_owned(),
            )
        })?;

        entity::oauth::ActiveModel::create_oauth(
            guild,
            "mastodon".to_owned(),
            true,
            instance,
            token,
        )
        .save(db)
        .await
        .map_err(|e| {
            log::error!("{:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to save new token to database".to_owned(),
            )
        })?;

        Ok::<(StatusCode, String), (StatusCode, String)>((
            StatusCode::OK,
            "Success! You can now close this window and continue configuring your bot.".to_owned(),
        ))
    } else {
        Err((
            StatusCode::NOT_ACCEPTABLE,
            "Failed to verify integrity of token. Please try again.".to_owned(),
        ))
    }
}

#[derive(Debug, Deserialize)]
pub struct RequestCodeQuery {
    guild: u64,
    instance: String,
    force_insecure: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct ResponseCodeQuery {
    code: String,
    state: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseState {
    pub guild: u64,
    pub instance: String,
}

#[derive(Debug, Deserialize)]
pub struct ResponseToken {
    access_token: String,
}

#[derive(Debug, Serialize)]
pub struct RequestToken {
    pub grant_type: String,
    pub code: String,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub scope: String,
}
