use crate::{
    defer_ephemeral,
    oauth::{common::OauthProvider, providers::MastodonProvider},
    require_mod,
    util::{check_permissions, database, delete_reply, oauth_safe, Context, Error},
};
use poise::serenity_prelude::{ActionRowComponent, ButtonStyle, CreateComponents, Message};

/// Publish OneWord story to Mastodon
#[poise::command(
    slash_command,
    ephemeral,
    guild_only,
    rename = "mastodon",
    context_menu_command = "🐘 post on mastodon",
    // required_permissions = "MANAGE_MESSAGES"
)]
pub async fn command(
    ctx: Context<'_>,
    #[description = "Story to publish (enter a link or ID)"] mut msg: Message,
) -> Result<(), Error> {
    require_mod!(ctx);
    defer_ephemeral!(ctx);

    let provider = ctx
        .data
        .oauth
        .get("mastodon")
        .expect("failed to retrieve mastodon provider");
    let guild_id = ctx
        .interaction
        .guild_id()
        .expect("failed to retrieve guild");
    let db = database(ctx);

    if !msg.author.id.eq(&ctx.framework.bot_id) || msg.embeds.is_empty() {
        return Err(Error::from(
            "Invalid message supplied, please select story summary!",
        ));
    }

    let oauth = oauth_safe(db, guild_id.0, "mastodon".to_owned()).await;
    if provider.has_missing || oauth.is_err() {
        return Err(Error::from("Mastodon is currently not configured!"));
    }
    let oauth = oauth.unwrap();

    if !oauth.active {
        return Err(Error::from("Mastodon is currently disabled!"));
    }

    let mut cc = CreateComponents(vec![]);
    let components = &msg.components;
    if !components.is_empty() {
        if let ActionRowComponent::Button(b) = &components[0].components[0] {
            if b.label == Some("🐘 mastodon".to_owned()) {
                return Err(Error::from("Message already published to mastodon!"));
            }
        }
    }

    let url = MastodonProvider::publish(oauth, msg.embeds[0].description.clone().unwrap())
        .await
        .map_err(|e| {
            log::error!("{e:?}");
            "Failed to publish to mastodon!"
        })?;

    cc.create_action_row(|r| {
        r.create_button(|b| {
            b.label("🐘 mastodon".to_owned())
                .style(ButtonStyle::Link)
                .url(url)
        })
    });

    msg.edit(&ctx.serenity_context, |m| m.set_components(cc))
        .await?;

    delete_reply(ctx).await?;

    Ok(())
}
