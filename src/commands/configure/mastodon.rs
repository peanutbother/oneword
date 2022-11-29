use crate::{
    oauth::{common::OauthProvider, providers::MastodonProvider},
    util::{database, edit_reply, guild_id, Context, Error},
};
use entity::{
    prelude::Oauth,
    sea_orm::{ColumnTrait, QueryFilter},
};
use poise::serenity_prelude::ButtonStyle;
use std::slice::Iter;

/// Configure twitter integration
#[poise::command(
    slash_command,
    ephemeral,
    guild_only,
    category = "setup",
    rename = "mastodon",
    required_permissions = "ADMINISTRATOR"
)]
pub async fn command(ctx: Context<'_>, #[description = "mastodon instance of user (default: mastodon.social)"] instance: Option<String>) -> Result<(), Error> {
    ctx.defer_response(true).await?;
    let db = database(ctx);
    let guild_id = guild_id(ctx);
    let url = MastodonProvider::get_url(Some(|url| {
        format!(
            "{url}?guild={guild_id}&instance={}",
            instance.or(Some("mastodon.social".to_owned())).unwrap()
        )
    }));
    // let guild = guild_safe(db, guild_id).await?;
    let oauth_config = Oauth::find_by_guild(guild_id)
        .filter(entity::oauth::Column::Name.eq("mastodon".to_string()))
        .one(db)
        .await?;
    let already_configured = oauth_config.is_some();
    let requirements = MastodonProvider::get_requirements();

    edit_reply(ctx, |b| {
        b
            .embed(|e| {
                e
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
                    );
                
                if requirements.has_missing {
                    e.field("requirements",
                        format!("{}\n{}",
                            format_iter(
                                requirements.missing.iter(),false
                            ),
                            format_iter(
                                requirements.fulfilled.iter(),true
                            ),
                        )
                        ,
                        false
                    );
                }
                e
            })
            .components(|c| {
                c.create_action_row(|ar| {
                    ar
                    .create_button(|b|{
                        b.style(ButtonStyle::Primary)
                        .custom_id("toggle_integration")
                        .label(if let Some(oauth_config) = oauth_config {if oauth_config.active {"Disable"} else {"Enable"}} else {"Disable"})
                        .disabled(!already_configured)
                    })
                    .create_button(|b| {
                        b
                            .style(ButtonStyle::Link)
                            .label("Request Token")
                            .disabled(requirements.has_missing)
                            .url(match url { Some(url) => url, None => "https://example.com".to_owned()})
                    })
                })
            })
    })
    .await?;

    Ok(())
}

fn format_iter(iter: Iter<(String, String)>, ok: bool) -> String {
    iter.map(|(_, requirement)| -> String {
        format!("{requirement} {}", if ok { "✅" } else { "❌" })
    })
    .collect::<Vec<String>>()
    .join("\n")
}
