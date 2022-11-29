use poise::serenity_prelude::Permissions;

use crate::util::{check_permissions, Context, Error};

mod mastodon;

/// Configure bot
#[poise::command(
    slash_command,
    category = "setup",
    ephemeral,
    subcommands("mastodon::command")
    // required_permissions = "ADMINISTRATOR",
)]
pub async fn configure(ctx: Context<'_>) -> Result<(), Error> {
    check_permissions(ctx, Permissions::ADMINISTRATOR)?;
    Ok(())
}
