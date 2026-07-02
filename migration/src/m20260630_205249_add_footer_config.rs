use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Guild::Table)
                    .add_column(
                        ColumnDef::new(Guild::HideUser)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(Guild::Table)
                    .add_column(
                        ColumnDef::new(Guild::HideDeletionInfo)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Guild::Table)
                    .drop_column(Guild::HideUser)
                    .drop_column(Guild::HideDeletionInfo)
                    .to_owned(),
            )
            .await
    }
}

#[derive(Iden)]
#[allow(unused)]
pub enum Guild {
    Table,
    Id,
    Active,
    RetainMessages,
    Oauth,
    HideUser,
    HideDeletionInfo,
}
