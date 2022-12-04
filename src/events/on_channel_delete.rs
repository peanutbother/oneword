use crate::util::Error;
use entity::{sea_orm::ActiveModelTrait, DatabaseConnection};
use poise::serenity_prelude::GuildChannel;

pub async fn handle(channel: &GuildChannel, db: &DatabaseConnection) -> Result<(), Error> {
    let channels = entity::channel::Entity::find_by_channel(channel.id.0)
        .all(db)
        .await?
        .into_iter()
        .map(Into::into)
        .collect::<Vec<entity::channel::ActiveModel>>();

    for channel in channels {
        channel.delete(db).await?;
    }

    Ok(())
}
