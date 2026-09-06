pub use sea_orm_migration::prelude::*;

pub mod enums;
mod m20251227_113110_init_table;
mod m20260328_120000_add_comment_guest_id;
mod m20260328_140000_add_oauth_redirect_uri;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
  fn migrations() -> Vec<Box<dyn MigrationTrait>> {
    vec![
      Box::new(m20251227_113110_init_table::Migration),
      Box::new(m20260328_120000_add_comment_guest_id::Migration),
      Box::new(m20260328_140000_add_oauth_redirect_uri::Migration),
    ]
  }
}
