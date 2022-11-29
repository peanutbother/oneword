use crate::util::{Data, Error};
use entity::sea_orm::{ColumnTrait, ModelTrait, QueryFilter};
use lazy_static::lazy_static;
use poise::serenity_prelude::{Context, Message};
use regex::Regex;
use std::collections::BTreeSet;

lazy_static! {
    static ref RE_W: Regex = Regex::new(r"(?m)\s").unwrap();
    static ref RE_W_END: Regex = Regex::new(r"(?m)(\s)([!?.…])").unwrap();
    static ref RE_W_COMMA: Regex = Regex::new(r"(?m)\s,").unwrap();
    // static ref RE_END: Regex = Regex::new(r"(?m)^[!?.…]$").unwrap();
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

    let guild_id = guild_id.unwrap().0;
    let channel_id = message.channel_id.0;
    let config = entity::channel::Entity::find_by_channel(channel_id)
        .filter(entity::channel::Column::GuildId.eq(guild_id))
        .one(db)
        .await?;

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
                .messages(ctx, |m| m.before(message.id).limit(100))
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
                .send_message(ctx, |m| {
                    m.embed(|e| {
                        e.description(words_sanitized.join(" "))
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
                            .footer(|f| {
                                f.text(format!(
                                    "Story Messages will {}be deleted | ended by {}",
                                    if retain_messages { "not " } else { "" },
                                    {
                                        let user = &messages
                                            .last()
                                            .expect("failed to get last user")
                                            .author;
                                        format!("{}#{}", user.name, user.discriminator)
                                    }
                                ))
                                .icon_url(
                                    message.author.avatar_url().unwrap_or_else(|| {
                                        ctx.cache.current_user().default_avatar_url()
                                    }),
                                )
                            })
                    })
                })
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
