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
                    .table(Oauth::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Oauth::Id)
                            .integer()
                            .not_null()
                            .primary_key()
                            .auto_increment(),
                    )
                    .col(ColumnDef::new(Oauth::GuildId).string().not_null())
                    .col(ColumnDef::new(Oauth::Name).string().not_null())
                    .col(ColumnDef::new(Oauth::Active).boolean().not_null().default(false))
                    .col(ColumnDef::new(Oauth::Instance).string().not_null())
                    .col(ColumnDef::new(Oauth::Data).json().not_null())
                    .foreign_key(
                        ForeignKey::create()
                        .name("guild_id")
                        .from(Oauth::Table, Oauth::GuildId)
                        .to(Guild::Table, Guild::Id)
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Oauth::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Oauth {
    Table,
    Id,
    GuildId,
    Name,
    Active,
    Instance,
    Data
}
