use migration::MigratorTrait;

pub async fn run(db: &entity::DatabaseConnection) -> Result<(), migration::DbErr> {
    migration::Migrator::up(db, None).await
}
