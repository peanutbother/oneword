use crate::{constants, oauth, util::Error};
use axum::{routing::get, Extension, Json, Router, Server};
use entity::DatabaseConnection;
use serde::Serialize;
use tokio::sync::OnceCell;

pub async fn init(db: OnceCell<DatabaseConnection>) -> Result<(), Error> {
    log::info!("initializing server");
    let app = Router::new();
    let app = base_routes_init(app);
    let app = oauth::router::init(app);
    let app = app.layer(Extension(db));

    Server::bind(&"0.0.0.0:3000".parse()?)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}

fn base_routes_init(app: Router) -> Router {
    app.route("/status", get(status))
}

#[derive(Debug, Serialize)]
struct Status<'a> {
    status: &'a str,
    version: &'a str,
}

async fn status() -> Json<Status<'static>> {
    Json(Status {
        //
        status: "ok",
        version: constants::VERSION,
    })
}
