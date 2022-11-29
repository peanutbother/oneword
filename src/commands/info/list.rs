use crate::util::{database, edit_reply, guild_id, Context, Error};
use poise::serenity_prelude::CreateEmbed;

/// List every active channel
#[poise::command(slash_command, category = "info", ephemeral, rename = "list")]
pub async fn command(ctx: Context<'_>) -> Result<(), Error> {
    let db = database(ctx);
    ctx.defer_response(true).await?;

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
