use crate::util::{guild_safe, Data, Error};
use entity::{
    guild::Model,
    sea_orm::{ColumnTrait, ModelTrait, QueryFilter},
};
use lazy_static::lazy_static;
use poise::serenity_prelude::{
    Context, CreateEmbed, CreateEmbedFooter, CreateMessage, GetMessages, Message,
};
use regex::Regex;
use std::collections::BTreeSet;

lazy_static! {
    static ref RE_W: Regex = Regex::new(r"(?m)\s").unwrap();
    static ref RE_W_END: Regex = Regex::new(r"(?m)(\s)([!?.…])").unwrap();
    static ref RE_W_COMMA: Regex = Regex::new(r"(?m)\s,").unwrap();
}

pub async fn handle(ctx: &Context, data: &Data, message: &Message) -> Result<(), Error> {
    let db = data.database.get().expect("failed to connect to database");

    let guild_id = &message.guild_id;
    if message.author.bot
        || guild_id.is_none()
        || !is_end_trigger(&message.content)
        || RE_W.find_iter(&message.content).count() > 1
    {
        return Ok(());
    }

    let guild_id = guild_id.unwrap().get();
    let channel_id = message.channel_id.get();
    let config = entity::channel::Entity::find_by_channel(channel_id)
        .filter(entity::channel::Column::GuildId.eq(guild_id))
        .one(db)
        .await?;

    let Model {
        hide_user,
        hide_deletion_info,
        ..
    } = guild_safe(db, guild_id).await?;

    if let Some(config) = config {
        // channel is set active
        let config: entity::channel::Model = config.into();
        let config = config
            .find_related(entity::guild::Entity)
            .one(db)
            .await?
            .expect("failed to retrieve guild");

        if config.active {
            let retain_messages = config.retain_messages;
            // TODO implement multiple handlers except just mastodon
            // get active provider configs
            // let config = config
            //     .find_related(entity::oauth::Entity)
            //     .filter(entity::oauth::Column::Active.eq(true))
            //     .all(db)
            //     .await?;
            // get provider names
            // config
            //     .iter()
            //     .map(|c| c.name.clone())
            //     .collect::<Vec<_>>()
            //     .join(",");

            let messages = message
                .channel_id
                .messages(ctx, GetMessages::new().before(message.id).limit(100))
                .await?;

            let messages = collect_messages(&messages, message.timestamp.timestamp());

            if messages.is_empty() {
                return Ok(());
            }

            let mut participants = messages
                .iter()
                .map(|m| m.author.id)
                .collect::<BTreeSet<_>>();
            let mut words = messages
                .iter()
                .map(|m| m.content.clone())
                .collect::<Vec<_>>();
            participants.insert(message.author.id);
            words.push(message.content.clone());

            let mut participants_users: Vec<String> = vec![];
            for participant in participants {
                participants_users.push(
                    participant
                        .to_user(ctx)
                        .await
                        .map_or_else(|_| "<unknown>".to_owned(), |u| format!("<@{}>", u.id)),
                );
            }

            let mut words_sanitized: Vec<String> = vec![];
            for word in words {
                let mut word = word;
                word = RE_W.replace(&word, " ").to_string();
                word = RE_W_END.replace(&word, "$2").to_string();
                word = RE_W_COMMA.replace(&word, ",").to_string();

                // if word
                words_sanitized.push(word);
            }

            message
                .channel_id
                .send_message(
                    ctx,
                    CreateMessage::new().embed(
                        CreateEmbed::new()
                            .description(words_sanitized.join(" "))
                            .fields(vec![
                                (
                                    format!("Participants ({})", participants_users.len()),
                                    participants_users.join(", "),
                                    true,
                                ),
                                (
                                    "Word Count".to_owned(),
                                    words_sanitized.len().to_string(),
                                    true,
                                ),
                            ])
                            .footer(
                                CreateEmbedFooter::new(if hide_deletion_info && hide_user {
                                    "".to_owned()
                                } else {
                                    format!(
                                        "{}{}",
                                        if !hide_deletion_info {
                                            format!(
                                                "Story Messages will {}be deleted{}",
                                                if retain_messages { "not " } else { "" },
                                                if !hide_user { " | " } else { "" }
                                            )
                                        } else {
                                            "".to_owned()
                                        },
                                        if !hide_user {
                                            format!(
                                                "ended by {}",
                                                message.author.display_name() // "ended by {}#{}",
                                                                              // message.author.name,
                                                                              // message.author.discriminator.as_ref().unwrap()
                                            )
                                        } else {
                                            "".to_owned()
                                        }
                                    )
                                })
                                .icon_url(
                                    message.author.avatar_url().unwrap_or_else(|| {
                                        ctx.cache.current_user().default_avatar_url()
                                    }),
                                ),
                            ),
                    ),
                )
                .await?;
            if !retain_messages {
                for message in messages {
                    message.delete(ctx).await?;
                }
            }
            return Ok(());
        }
    }

    Ok(())
}

fn collect_messages(messages: &Vec<Message>, before: i64) -> Vec<Message> {
    let collected: Vec<Message> = messages
        .clone()
        .into_iter()
        .filter_map(|m| {
            if m.timestamp.timestamp() < before {
                Some(m)
            } else {
                None
            }
        })
        .filter(|m| RE_W.find_iter(&m.content).count() < 2)
        .collect();
    // collected.reverse();

    let mut m: Vec<Message> = vec![];
    for message in &collected {
        if message.author.bot {
            m.reverse();
            return m;
        }
        m.push(message.to_owned());
    }
    // collected.reverse();
    collected
}

fn is_end_trigger(content: &String) -> bool {
    if content.is_empty() {
        return false;
    }

    let c = content
        .chars()
        .last()
        .expect("faield to get last character");

    match c.to_string().as_str() {
        "." => true,
        "…" => true,
        "!" => true,
        "?" => true,
        _ => false,
    }
}
