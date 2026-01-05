use sea_orm_migration::{prelude::*, schema::*};

use crate::enums::{CommentStatus, UserRole};

#[derive(Iden)]
enum Users {
  Table,
  Id,
  Nickname,
  Password,
  Email,
  Avatar,
  Role,
  Url,
  CreatedAt,
  UpdatedAt,
}

#[derive(Iden)]
enum Sites {
  Table,
  Id,
  Name,
  Url,
  Config,
  CreatedAt,
  UpdatedAt,
}

#[derive(Iden)]
enum Comments {
  Table,
  Id,
  UserId,
  SiteId,
  Rid,
  PagePath,
  Content,
  Status,
  Nick,
  Link,
  Email,
  Device,
  Location,
  IsSticky,
  UpVote,
  DownVote,
  CreatedAt,
  UpdatedAt,
  DeletedAt,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .create_table(
        Table::create()
          .table(Users::Table)
          .if_not_exists()
          .col(pk_auto(Users::Id))
          .col(string(Users::Nickname))
          .col(string(Users::Password))
          .col(string(Users::Email).unique_key())
          .col(string(Users::Avatar))
          .col(string(Users::Role).default(UserRole::Normal))
          .col(string(Users::Url))
          .col(date_time(Users::CreatedAt))
          .col(date_time(Users::UpdatedAt))
          .to_owned(),
      )
      .await?;

    manager
      .create_table(
        Table::create()
          .table(Sites::Table)
          .if_not_exists()
          .col(pk_auto(Sites::Id))
          .col(string(Sites::Name))
          .col(string(Sites::Url).unique_key())
          .col(json(Sites::Config))
          .col(date_time(Sites::CreatedAt))
          .col(date_time(Sites::UpdatedAt))
          .to_owned(),
      )
      .await?;

    manager
      .create_table(
        Table::create()
          .table(Comments::Table)
          .if_not_exists()
          .col(pk_auto(Comments::Id))
          .col(integer(Comments::UserId).null())
          .col(integer(Comments::SiteId))
          .col(integer(Comments::Rid).default(0))
          .col(string(Comments::PagePath))
          .col(text(Comments::Content))
          .col(string(Comments::Status).default(CommentStatus::Pending))
          .col(string(Comments::Nick))
          .col(string(Comments::Link))
          .col(string(Comments::Email))
          .col(string(Comments::Device))
          .col(string(Comments::Location))
          .col(boolean(Comments::IsSticky).default(false))
          .col(integer(Comments::UpVote).default(0))
          .col(integer(Comments::DownVote).default(0))
          .col(date_time(Comments::CreatedAt))
          .col(date_time(Comments::UpdatedAt))
          .col(date_time(Comments::DeletedAt).null())
          .foreign_key(
            ForeignKey::create()
              .name("fk-comments-user_id")
              .from(Comments::Table, Comments::UserId)
              .to(Users::Table, Users::Id)
              .on_delete(ForeignKeyAction::SetNull)
              .on_update(ForeignKeyAction::Cascade),
          )
          .foreign_key(
            ForeignKey::create()
              .name("fk-comments-site_id")
              .from(Comments::Table, Comments::SiteId)
              .to(Sites::Table, Sites::Id)
              .on_delete(ForeignKeyAction::Cascade)
              .on_update(ForeignKeyAction::Cascade),
          )
          .to_owned(),
      )
      .await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .drop_table(Table::drop().table(Comments::Table).to_owned())
      .await?;
    manager
      .drop_table(Table::drop().table(Sites::Table).to_owned())
      .await?;
    manager
      .drop_table(Table::drop().table(Users::Table).to_owned())
      .await
  }
}
