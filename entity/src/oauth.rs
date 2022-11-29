use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, Select, Set};

pub use crate::generated::oauth::*;
use crate::generated::prelude::Oauth;

impl ActiveModel {
    pub fn create_oauth<Data: Into<String>>(
        guild_id: u64,
        name: String,
        active: bool,
        instance: String,
        data: Data,
    ) -> Self {
        Self {
            guild_id: Set(guild_id.to_string()),
            name: Set(name),
            active: Set(active),
            instance: Set(instance),
            data: Set(data.into()),
            ..Default::default()
        }
    }
}

impl Oauth {
    pub fn find_by_guild(guild: u64) -> Select<Self> {
        Self::find().filter(Column::GuildId.eq(guild.to_string()))
    }
    pub fn find_by_name(name: String) -> Select<Self> {
        Self::find().filter(Column::Name.eq(name))
    }
    pub fn find_by_instance(instance: String) -> Select<Self> {
        Self::find().filter(Column::Instance.eq(instance))
    }
}
