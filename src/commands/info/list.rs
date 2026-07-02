use crate::{
    database, defer_ephemeral,
    util::{guild_id, Context, Error},
};
use poise::serenity_prelude::{CreateEmbed, EditInteractionResponse};

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

    let db = database!(ctx);
    let handles = entity::channel::Entity::find_by_guild(guild_id(&ctx))
        .all(db)
        .await?;

    ctx.interaction
        .edit_response(
            ctx,
            EditInteractionResponse::new().embed(
                CreateEmbed::new()
                    .title("active channels")
                    .description(if !handles.is_empty() {
                        "the following channels will be watched:"
                    } else {
                        "currently no active channel configs are present"
                    })
                    .fields(if !handles.is_empty() {
                        vec![(
                            "channel",
                            handles
                                .iter()
                                .map(|channel| format!("<#{}>", channel.channel_id))
                                .collect::<Vec<_>>()
                                .join("\n"),
                            true,
                        )]
                    } else {
                        vec![]
                    }),
            ),
        )
        .await?;

    Ok(())
}
