use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .alter_table(
        Table::alter()
          .table(OauthProviders::Table)
          .add_column(string(OauthProviders::RedirectUri).not_null().default(""))
          .to_owned(),
      )
      .await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .alter_table(
        Table::alter()
          .table(OauthProviders::Table)
          .drop_column(OauthProviders::RedirectUri)
          .to_owned(),
      )
      .await
  }
}

#[derive(Iden)]
enum OauthProviders {
  Table,
  RedirectUri,
}
