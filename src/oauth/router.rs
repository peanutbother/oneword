use super::check_requirements;
use super::common::OauthProvider;
use super::providers::MastodonProvider;
use super::ProviderRequirements;
use axum::response::Html;
use axum::routing::get;
use axum::Router;
use hyper::StatusCode;

pub fn init(router: Router) -> Router {
    let oauth_router = if check_requirements() {
        Router::new()
            // attach provider routers
            .nest("/mastodon", MastodonProvider::get_router())
    } else {
        log::warn!(
            "missing env: {} - oauth will not be available!",
            ProviderRequirements::from(vec![("OAUTH_URL_BASE", "oauth url base",)])
                .get_missing_list_str(),
        );
        Router::new().route(
            "/:provider/request",
            get(|| async {
                (
                    StatusCode::BAD_REQUEST,
                    Html("sorry, oauth is not configured".to_owned()),
                )
            }),
        )
    };

    router.nest("/oauth", oauth_router)
}
