use crate::{
    require_admin,
    util::{check_permissions, Context, Error},
};

mod footer;
mod mastodon;

/// Configure bot
#[poise::command(
    slash_command,
    ephemeral,
    guild_only,
    category = "setup",
    subcommands("mastodon::command", "footer::command")
    // required_permissions = "ADMINISTRATOR",
)]
pub async fn configure(ctx: Context<'_>) -> Result<(), Error> {
    require_admin!(ctx);
    Ok(())
}
