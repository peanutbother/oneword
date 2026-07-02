use crate::{
    oauth::{common::OauthProvider, providers::MastodonProvider},
    util::{  guild_id, Context, Error, check_permissions}, require_admin, defer_ephemeral,database
};
use entity::{
    prelude::Oauth,
    sea_orm::{ColumnTrait, QueryFilter},
};
use poise::serenity_prelude::{ButtonStyle, CreateActionRow, CreateButton, CreateEmbed, EditInteractionResponse, Message};
use std::{slice::Iter};

/// Configure mastodon integration
#[poise::command(
    slash_command,
    ephemeral,
    guild_only,
    category = "setup",
    rename = "mastodon",
    // required_permissions = "ADMINISTRATOR"
)]
pub async fn command(ctx: Context<'_>, #[description = "mastodon instance of user (default: mastodon.social)"] instance: Option<String>) -> Result<(), Error> {
    require_admin!(ctx);
    defer_ephemeral!(ctx);
    
    let db = database!(ctx);
    let guild_id = guild_id(&ctx);
    let url = MastodonProvider::get_url(Some(|url| {
        format!(
            "{url}?guild={guild_id}&instance={}",
            instance.unwrap_or("mastodon.social".to_owned())
        )
    })).unwrap_or("https://example.com".to_owned());
    // let guild = guild_safe(db, guild_id).await?;
    let oauth_config = Oauth::find_by_guild(guild_id)
        .filter(entity::oauth::Column::Name.eq("mastodon".to_string()))
        .one(db)
        .await?;
    let already_configured = oauth_config.is_some();
    let requirements = MastodonProvider::get_requirements();

    let message = ctx.interaction.edit_response(ctx,EditInteractionResponse::new()
            .embed(CreateEmbed::new()
                    .title("Configure Mastodon integration")
                    .description(
                        if !requirements.has_missing  {
                            format!("{}\n{}",
                                "This allows you to enable Mastodon integration to post stories on behalf of your mastodon account.",
                                if already_configured {
                                    "⚠️ There is already a user configured for this guild. This will overwrite this user on successful authentication!"
                                } else {""}
                            )
                        } else {
                            "⚠️ Right now, Mastodon integration is not available. Please contact bot owner to enable it for your guild.".to_owned()
                        }
                    ).fields(
                        if requirements.has_missing {
                            vec![       
                                (
                                    "requirements",
                                    format!("{}\n{}",
                                        format_iter(
                                            requirements.missing.iter(),false
                                        ),
                                        format_iter(
                                            requirements.fulfilled.iter(),true
                                        ),
                                    ),
                                    false
                                )
                            ]
                        } else {vec![]}
                    )
                
            )
            .components(
                create_components(if let Some(oauth_config) = oauth_config{oauth_config.active} else {false},already_configured,requirements.has_missing,url.clone())
            )
    )
    .await?;

    await_interaction(ctx, message).await?;

    Ok(())
}

fn create_components(oauth_active:bool,configured:bool,missing_requirements:bool,url: impl Into<String>) -> Vec<CreateActionRow> {
    vec![
                    CreateActionRow::Buttons(
                        vec![
                            CreateButton::new("toggle_integration")
                                .style(ButtonStyle::Primary)
                                .label(if oauth_active {"Disable"} else {"Enable"})
                                .disabled(!configured),
                            CreateButton::new_link(url)
                                .label("Request Token")
                                .disabled(missing_requirements)
                        ]
                    )
                ]
}

fn format_iter(iter: Iter<(String, String)>, ok: bool) -> String {
    iter.map(|(_, requirement)| -> String {
        format!("{requirement} {}", if ok { "✅" } else { "❌" })
    })
    .collect::<Vec<String>>()
    .join("\n")
}

// TODO add interaction handler
async fn await_interaction<'a>(_ctx:Context<'a>, _message: Message) -> Result<(), Error> {
    
    Ok(())
}
