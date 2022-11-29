use crate::util::{env_var, Error};
use entity::sea_orm::Database;
pub use entity::DatabaseConnection;
use tokio::sync::OnceCell;

#[derive(Debug)]
pub enum DatabaseError {
    MigrationError(migration::DbErr),
    EnvParseError(Error),
}

impl From<migration::DbErr> for DatabaseError {
    fn from(err: migration::DbErr) -> Self {
        DatabaseError::MigrationError(err)
    }
}
impl From<Error> for DatabaseError {
    fn from(err: Error) -> Self {
        DatabaseError::EnvParseError(err)
    }
}

pub async fn init() -> Result<OnceCell<DatabaseConnection>, DatabaseError> {
    log::info!("initializing database");
    let url: String = env_var("DATABASE_URL")?;
    let db = Database::connect(url).await?;

    log::info!("running migrations");
    crate::migration::run(&db).await?;

    Ok(OnceCell::new_with(Some(db)))
}
