use crate::util::{Data, Error};
use poise::serenity_prelude::{Context, FullEvent};

mod on_channel_delete;
mod on_guild_delete;
mod on_message;
mod on_ready;

pub async fn handle<'a>(ctx: &Context, event: &FullEvent, data: &Data) -> Result<(), Error> {
    let db = data
        .database
        .get()
        .expect("failed to retrieve database connection");

    match event {
        FullEvent::Ready { .. } => on_ready::handle().await,
        FullEvent::GuildDelete { incomplete, .. } => on_guild_delete::handle(incomplete, db).await,
        FullEvent::ChannelDelete { channel, messages } => {
            on_channel_delete::handle(channel, messages, db).await
        }
        FullEvent::Message { new_message } => on_message::handle(ctx, data, new_message).await,
        _ => Ok(()),
    }
}
