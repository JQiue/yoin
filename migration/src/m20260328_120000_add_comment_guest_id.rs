use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .alter_table(
        Table::alter()
          .table(Comments::Table)
          .add_column(string(Comments::GuestId).null())
          .to_owned(),
      )
      .await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .alter_table(
        Table::alter()
          .table(Comments::Table)
          .drop_column(Comments::GuestId)
          .to_owned(),
      )
      .await
  }
}

#[derive(Iden)]
enum Comments {
  Table,
  GuestId,
}
