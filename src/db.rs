use migration::{
  DbErr, Migrator, MigratorTrait,
  sea_orm::{Database, DatabaseBackend, DatabaseConnection},
};
use tracing::info;

pub async fn migrate(database_url: &str) -> Result<DatabaseConnection, DbErr> {
  let connection = Database::connect(database_url).await?;
  match connection.get_database_backend() {
    DatabaseBackend::Sqlite => info!("Using SQLite"),
    DatabaseBackend::MySql => info!("Using MySQL/MariaDB"),
    DatabaseBackend::Postgres => info!("Using PostgreSQL"),
    _ => todo!(),
  }
  Migrator::up(&connection, None).await?;
  Ok(connection)
}
