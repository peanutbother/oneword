use crate::{
    commands::{self, pre_command},
    events, oauth,
    util::{env_var, Data, Error},
};
use entity::DatabaseConnection;
use poise::{serenity_prelude::GatewayIntents, BoxFuture, Framework, FrameworkOptions};
use tokio::sync::OnceCell;

pub fn init(
    database: OnceCell<DatabaseConnection>,
) -> Result<poise::FrameworkBuilder<Data, Error>, Error> {
    log::info!("initializing bot");

    let token: String = env_var("DISCORD_TOKEN")?;
    let intents = GatewayIntents::non_privileged()
        | GatewayIntents::GUILD_VOICE_STATES
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::MESSAGE_CONTENT;
    let options = FrameworkOptions {
        commands: commands::prepare(),
        pre_command: |ctx| Box::pin(pre_command(ctx)),
        event_handler: |ctx, event, _framework, data| Box::pin(events::handle(ctx, event, data)),
        skip_checks_for_owners: true,
        ..Default::default()
    };

    let bot = Framework::builder()
        .token(token)
        .setup(
            move |ctx, _ready, framework| -> BoxFuture<'_, Result<Data, Error>> {
                Box::pin(async move {
                    log::info!("initializing bot commands");
                    poise::builtins::register_globally(ctx, &framework.options().commands).await?;

                    log::info!("initializing context");
                    let oauth = oauth::get_requirements();

                    Ok(Data { database, oauth })
                })
            },
        )
        .options(options)
        .intents(intents);

    Ok(bot)
}
