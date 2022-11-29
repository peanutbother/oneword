pub use sea_orm_migration::prelude::*;

mod m20221124_000001_create_guild;
mod m20221124_000002_create_channel;
mod m20221124_000003_create_oauth;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20221124_000001_create_guild::Migration),
            Box::new(m20221124_000002_create_channel::Migration),
            Box::new(m20221124_000003_create_oauth::Migration),
        ]
    }
}

// TODO: move session store from cookie to database?
