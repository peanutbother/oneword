mod router;

use self::router::{RequestToken, ResponseState};

use super::common::{OauthProvider, ProviderRequirements};
use crate::util::{env_var, Error};
use axum::{async_trait, routing::get, Router};
use oauth2::{
    basic::BasicClient, url::Url, AuthUrl, ClientId, ClientSecret, CsrfToken, RedirectUrl,
    ResponseType, Scope,
};
use serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize)]
struct Status {
    status: String,
    media_ids: Option<Vec<String>>,
    in_reply_to_id: Option<String>,
}

#[derive(Debug, Deserialize)]
struct StatusResponse {
    url: String,
}

#[derive(Debug)]
pub struct MastodonProvider;

#[async_trait]
impl OauthProvider<entity::oauth::Model> for MastodonProvider {
    fn get_url<F: FnOnce(String) -> String>(f: Option<F>) -> Option<String> {
        env_var::<String>("OAUTH_URL_BASE")
            .map(|base_url| {
                let url = format!("{base_url}/mastodon/request");
                if f.is_some() {
                    f.unwrap()(url)
                } else {
                    url
                }
            })
            .ok()
    }
    fn get_requirements() -> ProviderRequirements {
        vec![
            ("MASTODON_CLIENT_ID", "client id"),
            ("MASTODON_CLIENT_SECRET", "client secret"),
        ]
        .into()
    }

    fn get_router() -> Router {
        Router::new()
            .route("/request", get(router::request_token))
            .route("/callback", get(router::oauth_callback))
    }

    async fn publish(config: entity::oauth::Model, status: String) -> Result<String, Error> {
        let StatusResponse { url } = reqwest::Client::new()
            .post(&format!("{}/api/v1/statuses", config.instance))
            .json(&Status {
                status,
                media_ids: None,
                in_reply_to_id: None,
            })
            .bearer_auth(config.data)
            .send()
            .await?
            .json()
            .await?;

        Ok(url)
    }
}

impl MastodonProvider {
    fn create_client(instance_url: String) -> Result<BasicClient, Error> {
        Ok(
            // create mastodon oauth client
            BasicClient::new(
                ClientId::new(env_var("MASTODON_CLIENT_ID")?),
                Some(ClientSecret::new(env_var("MASTODON_CLIENT_SECRET")?)),
                // AuthUrl::new(format!("{instance_url}/oauth/authorize"))?,
                AuthUrl::new(format!("{instance_url}/oauth/authorize"))?,
                None,
            )
            .set_redirect_uri(RedirectUrl::new(format!(
                // use "urn:ietf:wg:oauth:2.0:oob" for manual code input
                // "urn:ietf:wg:oauth:2.0:oob"
                "{}/mastodon/callback",
                env_var::<String>("OAUTH_URL_BASE")?
            ))?),
        )
    }

    pub async fn request_auth(
        guild: u64,
        instance: String,
        force_insecure: bool,
    ) -> Result<(Url, CsrfToken), Error> {
        let instance = format!("http{}://{instance}", if force_insecure { "" } else { "s" });
        let client = MastodonProvider::create_client(instance.clone())?;

        let (url, token) = client
            // .exchange_client_credentials()
            .authorize_url(move || {
                CsrfToken::new(serde_json::to_string(&ResponseState { guild, instance }).unwrap())
            })
            .add_scope(Scope::new("write:statuses".to_owned()))
            .use_implicit_flow()
            .set_response_type(&ResponseType::new("code".to_owned()))
            .url();

        Ok((url, token))
    }

    // TODO check state - implement guild id signing
    pub async fn verify_code(_code: String, _state: u64) -> bool {
        true
    }

    pub fn request_token(instance: String, code: String) -> Result<(String, RequestToken), Error> {
        Ok((
            format!("{instance}/oauth/token"),
            RequestToken {
                client_id: env_var("MASTODON_CLIENT_ID")?,
                client_secret: env_var("MASTODON_CLIENT_SECRET")?,
                grant_type: "authorization_code".to_owned(),
                code,
                redirect_uri: format!(
                    "{}/mastodon/callback",
                    // use "urn:ietf:wg:oauth:2.0:oob" for manual code input
                    env_var::<String>("OAUTH_URL_BASE")?
                ),
                scope: "write:statuses".to_owned(),
            },
        ))
    }
}
