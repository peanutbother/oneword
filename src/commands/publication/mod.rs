use crate::{
    require_mod,
    util::{check_permissions, Context, Error},
};

mod mastodon;

/// Publish a OneWord story
///
/// As of right now, only Mastodon is available.
/// To publish a OneWord story right click on the summary post of OneWord,
/// click on `Apps` and then click on `publish to mastodon`.
///
/// Required permissions: `Manage Messages`.
#[poise::command(
    slash_command,
    ephemeral,
    guild_only,
    subcommands("mastodon::command")
    // required_permissions = "ADMINISTRATOR",
)]
pub async fn publish(ctx: Context<'_>) -> Result<(), Error> {
    require_mod!(ctx);
    Ok(())
}
