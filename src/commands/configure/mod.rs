use crate::util::{Context, Error};

mod mastodon;

/// Configure bot
#[poise::command(
    slash_command,
    category = "setup",
    ephemeral,
    required_permissions = "ADMINISTRATOR",
    subcommands("mastodon::command")
)]
pub async fn configure(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}
