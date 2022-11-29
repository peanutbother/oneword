use entity::sea_orm::ActiveModelTrait;

// use crate::util::edit_reply;
// use super::update;
use crate::util::database;
use crate::util::edit_reply;
use crate::util::guild_safe;
use crate::util::Context;
use crate::util::Error;

/// activate bot for this server
#[poise::command(
    slash_command,
    ephemeral,
    guild_only,
    category = "setup",
    rename = "activate",
    required_permissions = "ADMINISTRATOR"
)]
pub async fn command(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer_response(true).await?;

    let db = database(ctx);
    let guild_id = ctx
        .interaction
        .guild_id()
        .expect("command cannot be run outside of guild")
        .0;
    let guild = guild_safe(db, guild_id).await?;
    entity::guild::ActiveModel::update_guild(guild_id, true, guild.retain_messages, guild.oauth)
        .update(db)
        .await?;

    edit_reply(ctx, |b| b.content("activated bot for this guild!")).await?;

    Ok(())
}