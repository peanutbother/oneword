use crate::{
    defer_ephemeral, require_admin,
    util::{check_permissions, edit_reply, Context, Error},
};
use entity::sea_orm::{ActiveModelTrait, ColumnTrait, PaginatorTrait, QueryFilter};
use poise::serenity_prelude::GuildChannel;

/// Activate a channel for OneWord to look after
#[poise::command(
    slash_command,
    ephemeral,
    guild_only,
    category = "setup",
    rename = "watch",
    // required_permissions = "ADMINISTRATOR"
)]
pub async fn command(
    ctx: Context<'_>,
    #[description = "channel to watch"]
    #[channel_types("Text")]
    channel: GuildChannel,
) -> Result<(), Error> {
    require_admin!(ctx);
    defer_ephemeral!(ctx);

    let (updated, channels) = watch_save(ctx, channel.clone()).await?;

    edit_reply(ctx, |b| {
        b.embed(|e| {
            e.title(if updated {
                "successfully updated settings"
            } else {
                "nothing to change"
            })
            .description("The following channels will be watched:")
            .field(
                "channel",
                channels
                    .iter()
                    .map(|channel| format!("<#{}>", channel.channel_id))
                    .collect::<Vec<_>>()
                    .join("\n"),
                true,
            )
        })
    })
    .await?;

    Ok(())
}

async fn watch_save(
    ctx: Context<'_>,
    channel: GuildChannel,
) -> Result<(bool, Vec<entity::channel::Model>), Error> {
    let db = ctx
        .data
        .database
        .get()
        .expect("database connection not yet initialized");

    let guild_id = ctx
        .interaction
        .guild_id()
        .expect("failed to get guild id")
        .0;
    let channel = channel.id.0;

    let guild = entity::channel::ActiveModel::update_channel(guild_id, channel);
    let count = entity::channel::Entity::find_by_guild(guild_id)
        .filter(entity::channel::Column::ChannelId.eq(channel.to_string()))
        .count(db)
        .await?;

    if count == 0 {
        guild.save(db).await?;
    }

    let channels = entity::channel::Entity::find_by_guild(guild_id)
        .all(db)
        .await?;

    Ok((count == 0, channels))
}
