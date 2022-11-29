use crate::util::{edit_reply, Context, Error};
use entity::sea_orm::{ActiveModelTrait, ColumnTrait, PaginatorTrait, QueryFilter};
use poise::serenity_prelude::GuildChannel;

/// activate a channel with a certain role to add
#[poise::command(
    slash_command,
    category = "setup",
    ephemeral,
    guild_only,
    rename = "watch",
    required_permissions = "ADMINISTRATOR"
)]
pub async fn command(
    ctx: Context<'_>,
    #[description = "channel to watch"] channel: GuildChannel,
) -> Result<(), Error> {
    ctx.defer_response(true).await?;
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
