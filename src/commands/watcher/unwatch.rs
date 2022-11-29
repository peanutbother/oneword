use crate::util::{database, edit_reply, Context, Error};
use entity::sea_orm::ActiveModelTrait;
use poise::{serenity_prelude::ChannelId, AutocompleteChoice};

/// Disable a channel / role binding
#[poise::command(
    slash_command,
    category = "setup",
    ephemeral,
    guild_only,
    rename = "unwatch",
    required_permissions = "ADMINISTRATOR"
)]
pub async fn command(
    ctx: Context<'_>,
    #[description = "which assignment to delete"]
    #[autocomplete = "unwatch_autocomplete"]
    channel: u64,
) -> Result<(), Error> {
    ctx.defer_response(true).await?;
    unwatch_save(ctx, channel).await?;

    edit_reply(ctx, |b| {
        b.embed(|e| {
            //
            e.title("successfully updated settings")
                .description("the following channel will no longer be watched")
                .field("channel", format!("<#{channel}>"), true)
        })
    })
    .await?;
    Ok(())
}

pub async fn unwatch_save(ctx: Context<'_>, id: u64) -> Result<(), Error> {
    let db = database(ctx);
    let entry = entity::channel::Entity::find_by_channel(id).one(db).await?;

    if let Some(entry) = entry {
        let model: entity::channel::ActiveModel = entry.into();
        model.delete(db).await?;

        Ok(())
    } else {
        Err(Error::from(format!("<#{id}> is not yet watched")))
    }
}

async fn unwatch_autocomplete(
    ctx: Context<'_>,
    _partial: &str,
) -> impl Iterator<Item = AutocompleteChoice<u64>> {
    let db = database(ctx);
    let guild = ctx
        .interaction
        .guild_id()
        .expect("cannot run this command outside of a guild");

    let channels = guild
        .channels(ctx.serenity_context.http.clone())
        .await
        .expect("failed to fetch guild channels");

    let connections = entity::channel::Entity::find_by_guild(guild.0)
        .all(db)
        .await
        .expect("failed to get database entries");

    connections.into_iter().map(move |m| {
        let channel = channels
            .get(&ChannelId::from(m.channel_id.parse::<u64>().unwrap()))
            .expect("failed to retrieve channel");

        let name = format!("#{}", channel.name);

        AutocompleteChoice {
            name,
            value: channel.id.0,
        }
    })
}
