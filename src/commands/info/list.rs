use crate::{
    defer_ephemeral,
    util::{database, edit_reply, guild_id, Context, Error},
};
use poise::serenity_prelude::CreateEmbed;

/// List every active channel
#[poise::command(
    slash_command,
    ephemeral,
    guild_only,
    category = "info",
    rename = "list"
)]
pub async fn command(ctx: Context<'_>) -> Result<(), Error> {
    defer_ephemeral!(ctx);

    let db = database(ctx);
    let handles = entity::channel::Entity::find_by_guild(guild_id(ctx))
        .all(db)
        .await?;

    edit_reply(ctx, |b| {
        b.add_embed(if !handles.is_empty() {
            CreateEmbed::default()
                .title("active channels")
                .description("the following channels will be watched:")
                .field(
                    "channel",
                    handles
                        .iter()
                        .map(|channel| format!("<#{}>", channel.channel_id))
                        .collect::<Vec<_>>()
                        .join("\n"),
                    true,
                )
                .to_owned()
        } else {
            // fallback embed
            CreateEmbed::default()
                .title("active channels")
                .description("currently no active channel configs are present")
                .to_owned()
        })
    })
    .await?;

    Ok(())
}
