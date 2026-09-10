use migration::{
  DbErr, Migrator, MigratorTrait,
  sea_orm::{Database, DatabaseBackend, DatabaseConnection},
};
use tracing::info;

/// 建立连接并记录后端类型，不执行迁移。
pub async fn connect(database_url: &str) -> Result<DatabaseConnection, DbErr> {
  let connection = Database::connect(database_url).await?;
  match connection.get_database_backend() {
    DatabaseBackend::Sqlite => info!("Using SQLite"),
    DatabaseBackend::MySql => info!("Using MySQL/MariaDB"),
    DatabaseBackend::Postgres => info!("Using PostgreSQL"),
    _ => todo!(),
  }
  Ok(connection)
}

/// 建立连接并执行迁移。
pub async fn migrate(database_url: &str) -> Result<DatabaseConnection, DbErr> {
  let connection = connect(database_url).await?;
  Migrator::up(&connection, None).await?;
  Ok(connection)
}
