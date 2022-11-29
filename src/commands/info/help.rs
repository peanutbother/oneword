use crate::constants;
use crate::util::Context;
use crate::util::Error;
use poise::samples::HelpConfiguration;

/// Display bot help
#[poise::command(slash_command, category = "info", ephemeral, rename = "help")]
pub async fn command(
    ctx: Context<'_>,
    #[description = "get help for a specific command"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
) -> Result<(), Error> {
    ctx.defer_response(true).await?;

    let footer_text = format!("{} v{}", constants::NAME, constants::VERSION);
    let help_config = HelpConfiguration {
        extra_text_at_bottom: footer_text.as_str(),
        ..Default::default()
    };

    let ctx = ctx.into();
    match command {
        Some(command) => poise::builtins::help(ctx, Some(command.as_str()), help_config).await,
        None => poise::builtins::help(ctx, None, help_config).await,
    }?;

    Ok(())
}
