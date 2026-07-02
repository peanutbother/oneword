use crate::database;
use crate::defer_ephemeral;
use crate::require_admin;
use crate::util::check_permissions;
use crate::util::guild_safe;
use crate::util::Context;
use crate::util::Error;
use entity::sea_orm::ActiveModelTrait;
use poise::serenity_prelude::EditInteractionResponse;

/// Activate bot for this server
#[poise::command(
    slash_command,
    ephemeral,
    guild_only,
    category = "setup",
    rename = "activate",
    // required_permissions = "ADMINISTRATOR"
)]
pub async fn command(ctx: Context<'_>) -> Result<(), Error> {
    require_admin!(ctx);
    defer_ephemeral!(ctx);

    let db = database!(ctx);
    let guild_id = ctx
        .interaction
        .guild_id
        .as_ref()
        .expect("command cannot be run outside of guild")
        .get();
    let guild = guild_safe(db, guild_id).await?;
    entity::guild::ActiveModel::update_guild(
        guild_id,
        true,
        guild.retain_messages,
        guild.oauth,
        guild.hide_user,
        guild.hide_deletion_info,
    )
    .update(db)
    .await?;

    ctx.interaction
        .edit_response(
            ctx,
            EditInteractionResponse::new().content("activated bot for this guild"),
        )
        .await?;

    Ok(())
}
