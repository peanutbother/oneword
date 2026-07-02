pub use sea_orm_migration::prelude::*;

mod m20221124_000001_create_guild;
mod m20221124_000002_create_channel;
mod m20221124_000003_create_oauth;
mod m20260630_205249_add_footer_config;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20221124_000001_create_guild::Migration),
            Box::new(m20221124_000002_create_channel::Migration),
            Box::new(m20221124_000003_create_oauth::Migration),
            Box::new(m20260630_205249_add_footer_config::Migration),
        ]
    }
}

// TODO: move session store from cookie to database?
