mod bot;
mod commands;
mod constants;
mod db;
mod events;
mod logger;
mod migration;
mod oauth;
mod server;
mod util;

#[tokio::main]
async fn main() {
    logger::init();
    log::info!(
        "{} v{} - {}",
        constants::NAME,
        constants::VERSION,
        constants::REPOSITORY
    );

    let db = db::init().await.expect("failed to setup database");
    let bot = bot::init(db.clone()).expect("failed to setup bot");
    let server = server::init(db);
    let bot = bot.run_autosharded();

    tokio::select! {
        server = server => {
            server.expect("server exited unexpectedly");
        }
        bot = bot => {
            bot.expect("bot exited unexpectedly");
        }
    };
}
