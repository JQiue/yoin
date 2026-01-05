pub use sea_orm_migration::prelude::*;

mod m20251227_113110_init_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
  fn migrations() -> Vec<Box<dyn MigrationTrait>> {
    vec![Box::new(m20251227_113110_init_table::Migration)]
  }
}
