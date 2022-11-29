use sea_orm_migration::prelude::*;
use crate::{m20221124_000001_create_guild::Guild};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Channel::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Channel::Id)
                            .integer()
                            .auto_increment()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Channel::GuildId)
                            .string()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(Channel::ChannelId)
                            .string()
                            .not_null()
                        )
                        .foreign_key(
                            ForeignKey::create()
                            .name("guild_id")
                            .from(Channel::Table, Channel::GuildId)
                            .to(Guild::Table, Guild::Id)
                        )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Channel::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum Channel {
    Table,
    Id,
    GuildId,
    ChannelId,
}
