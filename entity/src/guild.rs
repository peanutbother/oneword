use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, Select, Set};

pub use crate::generated::guild::*;
use crate::generated::prelude::Guild;

impl ActiveModel {
    pub fn update_guild(guild: u64, active: bool, retain_messages: bool, oauth: bool) -> Self {
        Self {
            id: Set(guild.to_string()),
            active: Set(active),
            retain_messages: Set(retain_messages),
            oauth: Set(oauth),
        }
    }
}

impl Guild {
    pub fn find_by_id(guild: u64) -> Select<Self> {
        Self::find().filter(Column::Id.eq(guild.to_string()))
    }
}
