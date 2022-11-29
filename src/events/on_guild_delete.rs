use crate::util::Error;
use entity::sea_orm::ActiveModelTrait;
use entity::DatabaseConnection;
use poise::serenity_prelude::UnavailableGuild;

pub async fn handle(incomplete: &UnavailableGuild, db: &DatabaseConnection) -> Result<(), Error> {
    let guild = incomplete.id.0;

    let channels = entity::channel::Entity::find_by_guild(guild)
        .all(db)
        .await?
        .into_iter()
        .map(Into::into)
        .collect::<Vec<entity::channel::ActiveModel>>();

    for channel in channels {
        channel.delete(db).await?;
    }

    let guild = entity::guild::Entity::find_by_id(guild).one(db).await?;

    if let Some(guild) = guild {
        let guild: entity::guild::ActiveModel = guild.into();
        guild.delete(db).await?;
    }

    Ok(())
}
