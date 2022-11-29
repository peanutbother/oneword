use entity::{
    sea_orm::{ColumnTrait, QueryFilter},
    DatabaseConnection,
};
use poise::{
    serenity_prelude::{CreateInteractionResponseFollowup, Message},
    ApplicationContext,
};
use std::str::FromStr;
use tokio::sync::OnceCell;

use crate::oauth::OauthRequirements;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = ApplicationContext<'a, Data, Error>;
pub type PoiseContext<'a> = poise::Context<'a, Data, Error>;
pub type Command = poise::Command<Data, Error>;

pub fn env_var<T: FromStr>(name: &str) -> Result<T, Error>
where
    T::Err: std::fmt::Display,
{
    Ok(std::env::var(name)
        .map_err(|_| format!("Missing {}", name))?
        .parse()
        .map_err(|e| format!("Invalid {}: {}", name, e))?)
}

pub fn has_env(name: &str) -> bool {
    std::env::vars()
        .find(|(k, v)| k == name && !v.is_empty())
        .is_some()
}

#[derive(Debug)]
pub struct Data {
    pub database: OnceCell<DatabaseConnection>,
    pub oauth: OauthRequirements,
}

pub async fn edit_reply<'a, F>(ctx: Context<'_>, b: F) -> Result<Message, Error>
where
    for<'b> F: FnOnce(
        &'b mut CreateInteractionResponseFollowup<'a>,
    ) -> &'b mut CreateInteractionResponseFollowup<'a>,
{
    let interaction = match ctx.interaction {
        poise::ApplicationCommandOrAutocompleteInteraction::ApplicationCommand(interaction) => {
            interaction
        }
        poise::ApplicationCommandOrAutocompleteInteraction::Autocomplete(_) => {
            panic!("cannot edit in autocomplete context!")
        }
    };

    return interaction
        .create_followup_message(ctx.serenity_context.http.clone(), b)
        .await
        .map_err(|err| err.into());
}

pub async fn delete_reply<'a>(ctx: Context<'_>) -> Result<(), Error> {
    let interaction = match ctx.interaction {
        poise::ApplicationCommandOrAutocompleteInteraction::ApplicationCommand(interaction) => {
            interaction
        }
        poise::ApplicationCommandOrAutocompleteInteraction::Autocomplete(_) => {
            panic!("cannot edit in autocomplete context!")
        }
    };

    interaction
        .delete_original_interaction_response(ctx.serenity_context.http.clone())
        .await?;

    Ok(())
}

pub fn database(ctx: Context<'_>) -> &'_ DatabaseConnection {
    ctx.data
        .database
        .get()
        .expect("failed to retrieve database connection")
}

pub fn guild_id(ctx: Context<'_>) -> u64 {
    ctx.interaction
        .guild_id()
        .expect("this command cannot be run outside of guilds")
        .0
}

#[allow(unused)]
pub async fn guild_safe(
    db: &DatabaseConnection,
    guild_id: u64,
) -> Result<entity::guild::Model, Error> {
    Ok(entity::guild::Entity::find_by_id(guild_id)
        .one(db)
        .await?
        .expect("failed to retrieve guild"))
}

#[allow(unused)]
pub async fn oauth_safe(
    db: &DatabaseConnection,
    guild_id: u64,
    name: String,
) -> Result<entity::oauth::Model, Error> {
    entity::oauth::Entity::find_by_guild(guild_id)
        .filter(entity::oauth::Column::Name.eq(name))
        .one(db)
        .await?
        .ok_or(Error::from("failed to retrieve guild"))
}

pub fn into_application_ctx(ctx: PoiseContext) -> Context {
    let ctx = match ctx {
        poise::Context::Application(ctx) => ctx,
        poise::Context::Prefix(_) => unimplemented!(),
    };
    ctx
}

pub fn check_permissions(
    ctx: Context<'_>,
    required_permissions: poise::serenity_prelude::Permissions,
) -> Result<(), Error> {
    let permissions = ctx
        .interaction
        .member()
        .unwrap()
        .permissions
        .expect("permission check cannot be run outside of interactions");

    if permissions.bits() > required_permissions.bits()
        && !ctx
            .framework
            .options
            .owners
            .contains(&ctx.interaction.user().id)
    {
        Err(Error::from("insufficient permissions!"))
    } else {
        Ok(())
    }
}
