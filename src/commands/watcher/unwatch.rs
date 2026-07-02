use crate::{
    database, defer_ephemeral, require_admin,
    util::{check_permissions, Context, Error},
};
use entity::sea_orm::ActiveModelTrait;
use poise::serenity_prelude::{
    AutocompleteChoice, ChannelId, CreateEmbed, EditInteractionResponse, MessageId,
};

/// Disable a channel for OneWord to look after
#[poise::command(
    slash_command,
    ephemeral,
    guild_only,
    category = "setup",
    rename = "unwatch",
    // required_permissions = "ADMINISTRATOR"
)]
pub async fn command(
    ctx: Context<'_>,
    #[description = "which assignment to delete"]
    #[autocomplete = "unwatch_autocomplete"]
    channel: u64,
) -> Result<(), Error> {
    require_admin!(ctx);
    defer_ephemeral!(ctx);

    unwatch_save(ctx, channel).await?;

    ctx.interaction
        .edit_response(
            ctx,
            EditInteractionResponse::new().embed(
                CreateEmbed::new()
                    .title("successfully updated settings")
                    .description("the following channel will no longer be watched")
                    .field("channel", format!("<#{channel}>"), true),
            ),
        )
        .await?;
    Ok(())
}

pub async fn unwatch_save(ctx: Context<'_>, id: u64) -> Result<(), Error> {
    let db = database!(ctx);
    let entry = entity::channel::Entity::find_by_channel(id).one(db).await?;

    if let Some(entry) = entry {
        let model: entity::channel::ActiveModel = entry.into();
        model.delete(db).await?;

        Ok(())
    } else {
        Err(Error::from(format!(
            "{} is not yet watched",
            MessageId::new(id).link(ctx.channel_id(), ctx.guild_id())
        )))
    }
}

async fn unwatch_autocomplete(
    ctx: Context<'_>,
    _partial: &str,
) -> impl Iterator<Item = AutocompleteChoice> {
    let db = database!(ctx);
    let guild = ctx
        .interaction
        .guild_id
        .as_ref()
        .expect("cannot run this command outside of a guild");

    let channels = guild
        .channels(ctx.serenity_context.http.clone())
        .await
        .expect("failed to fetch guild channels");

    let connections = entity::channel::Entity::find_by_guild(guild.get())
        .all(db)
        .await
        .expect("failed to get database entries");

    connections.into_iter().map(move |m| {
        let channel = channels
            .get(&ChannelId::from(m.channel_id.parse::<u64>().unwrap()))
            .expect("failed to retrieve channel");

        let name = format!("#{}", channel.name);

        AutocompleteChoice::new(name, channel.id.get())
    })
}
