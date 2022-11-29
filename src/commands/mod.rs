use crate::util::{into_application_ctx, Command, PoiseContext};
use entity::sea_orm::{ActiveModelTrait, Set};

pub mod activation;
pub mod configure;
pub mod info;
pub mod publication;
pub mod watcher;

pub async fn pre_command<'a>(ctx: PoiseContext<'a>) {
    let ctx = into_application_ctx(ctx);

    let db = crate::util::database(ctx);
    let guild_id = ctx
        .interaction
        .guild_id()
        .expect("failed to get guild id")
        .0;
    let guild = entity::guild::ActiveModel {
        id: Set(guild_id.to_string()),
        active: Set(false),
        retain_messages: Set(true),
        oauth: Set(false),
        ..Default::default()
    };
    guild.insert(db).await.ok();
}

pub fn prepare() -> Vec<Command> {
    vec![
        activation::activate(),
        activation::deactivate(),
        configure::configure(),
        info::list(),
        info::help(),
        publication::publish(),
        watcher::watch(),
        watcher::unwatch(),
    ]
}
