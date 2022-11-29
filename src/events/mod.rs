use crate::util::{Data, Error};
use poise::{serenity_prelude::Context, Event};

mod on_channel_delete;
mod on_guild_delete;
mod on_message;
mod on_ready;

pub async fn handle<'a>(ctx: &Context, event: &Event<'a>, data: &Data) -> Result<(), Error> {
    let db = data
        .database
        .get()
        .expect("failed to retrieve database connection");

    match event {
        Event::Ready { .. } => on_ready::handle().await,
        Event::GuildDelete { incomplete, .. } => on_guild_delete::handle(incomplete, db).await,
        Event::ChannelDelete { channel } => on_channel_delete::handle(channel, db).await,
        Event::Message { new_message } => on_message::handle(ctx, data, new_message).await,
        _ => Ok(()),
    }
}
