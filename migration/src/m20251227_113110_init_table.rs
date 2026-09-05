use sea_orm_migration::{prelude::*, schema::*};

use crate::enums::CommentStatus;

#[derive(Iden)]
enum Users {
  Table,
  Id,
  Nickname,
  Password,
  Email,
  Avatar,
  Website,
  CreatedAt,
  UpdatedAt,
}

#[derive(Iden)]
enum UserIdentities {
  Table,
  Id,
  UserId,
  Provider,
  ProviderUserId,
  Email,
  Profile,
  CreatedAt,
  UpdatedAt,
}

#[derive(Iden)]
enum OauthProviders {
  Table,
  Id,
  SiteId,
  Enabled,
  ProviderCode,
  ClientId,
  ClientSecret,
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
  ThreadId,
  ParentId,
  PagePath,
  Content,
  Status,
  Nickname,
  Website,
  Email,
  Avatar,
  Device,
  Location,
  IsSticky,
  IsAnonymous,
  IsPrivate,
  CreatedAt,
  UpdatedAt,
  DeletedAt,
}

#[derive(Iden)]
enum Reactions {
  Table,
  Id,
  SiteId,
  TargetType,
  CommentId,
  PagePath,
  ActorType,
  ActorId,
  Type,
  TargetKey,
  CreatedAt,
}

#[derive(Iden)]
enum CommentSubscriptions {
  Table,
  Id,
  SiteId,
  PagePath,
  EventType,
  UserId,
  Email,
  IsActive,
  CreatedAt,
  UpdatedAt,
}

#[derive(Iden)]
enum Roles {
  Table,
  Id,
  Name,
  Description,
  CreatedAt,
  UpdatedAt,
}

#[derive(Iden)]
enum RolePermissions {
  Table,
  Id,
  RoleId,
  PermissionId,
  CreatedAt,
  UpdatedAt,
}

#[derive(Iden)]
enum Permissions {
  Table,
  Id,
  Name,
  Description,
  CreatedAt,
  UpdatedAt,
}

#[derive(Iden)]
enum UserRoleBindings {
  Table,
  Id,
  UserId,
  RoleId,
  ScopeType,
  ScopeId,
  CreatedAt,
  UpdatedAt,
}

#[derive(Iden)]
enum ModerationProviders {
  Table,
  Id,
  SiteId,
  ProviderKind,
  Enabled,
  Config,
  CreatedAt,
  UpdatedAt,
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
          .col(big_pk_auto(Users::Id))
          .col(string(Users::Nickname))
          .col(string(Users::Password))
          .col(string(Users::Email).unique_key())
          .col(string(Users::Avatar))
          .col(string(Users::Website))
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
          .col(big_pk_auto(Sites::Id))
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
          .table(OauthProviders::Table)
          .if_not_exists()
          .col(big_pk_auto(OauthProviders::Id))
          .col(big_integer(OauthProviders::SiteId).null())
          .col(string(OauthProviders::ProviderCode))
          .col(boolean(OauthProviders::Enabled).default(true))
          .col(string(OauthProviders::ClientId))
          .col(string(OauthProviders::ClientSecret))
          .col(date_time(OauthProviders::CreatedAt))
          .col(date_time(OauthProviders::UpdatedAt))
          .foreign_key(
            ForeignKey::create()
              .name("fk-oauth-providers-site_id")
              .from(OauthProviders::Table, OauthProviders::SiteId)
              .to(Sites::Table, Sites::Id)
              .on_delete(ForeignKeyAction::Cascade)
              .on_update(ForeignKeyAction::Cascade),
          )
          .to_owned(),
      )
      .await?;

    manager
      .create_table(
        Table::create()
          .table(ModerationProviders::Table)
          .if_not_exists()
          .col(big_pk_auto(ModerationProviders::Id))
          .col(big_integer(ModerationProviders::SiteId))
          .col(string(ModerationProviders::ProviderKind))
          .col(boolean(ModerationProviders::Enabled).default(true))
          .col(json(ModerationProviders::Config))
          .col(date_time(ModerationProviders::CreatedAt))
          .col(date_time(ModerationProviders::UpdatedAt))
          .foreign_key(
            ForeignKey::create()
              .name("fk-moderation-providers-site_id")
              .from(ModerationProviders::Table, ModerationProviders::SiteId)
              .to(Sites::Table, Sites::Id)
              .on_delete(ForeignKeyAction::Cascade)
              .on_update(ForeignKeyAction::Cascade),
          )
          .to_owned(),
      )
      .await?;

    manager
      .create_table(
        Table::create()
          .table(UserIdentities::Table)
          .if_not_exists()
          .col(big_pk_auto(UserIdentities::Id))
          .col(big_integer(UserIdentities::UserId))
          .col(string(UserIdentities::Provider))
          .col(string(UserIdentities::ProviderUserId))
          .col(string(UserIdentities::Email).null())
          .col(json(UserIdentities::Profile).null())
          .col(date_time(UserIdentities::CreatedAt))
          .col(date_time(UserIdentities::UpdatedAt))
          .index(
            Index::create()
              .name("idx-user_identities-provider-subject-unique")
              .table(UserIdentities::Table)
              .col(UserIdentities::Provider)
              .col(UserIdentities::ProviderUserId)
              .unique(),
          )
          .foreign_key(
            ForeignKey::create()
              .name("fk-user_identities-user_id")
              .from(UserIdentities::Table, UserIdentities::UserId)
              .to(Users::Table, Users::Id)
              .on_delete(ForeignKeyAction::Cascade)
              .on_update(ForeignKeyAction::Cascade),
          )
          .to_owned(),
      )
      .await?;

    manager
      .create_table(
        Table::create()
          .table(Comments::Table)
          .if_not_exists()
          .col(big_pk_auto(Comments::Id))
          .col(big_integer(Comments::UserId).null())
          .col(big_integer(Comments::SiteId))
          .col(big_integer(Comments::ThreadId).null())
          .col(big_integer(Comments::ParentId).null())
          .col(string(Comments::PagePath))
          .col(text(Comments::Content))
          .col(string(Comments::Status).default(CommentStatus::Pending))
          .col(string(Comments::Nickname))
          .col(string(Comments::Avatar))
          .col(string(Comments::Website))
          .col(string(Comments::Email))
          .col(string(Comments::Device))
          .col(string(Comments::Location))
          .col(boolean(Comments::IsSticky).default(false))
          .col(boolean(Comments::IsAnonymous).default(false))
          .col(boolean(Comments::IsPrivate).default(false))
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
      .await?;
    manager
      .create_table(
        Table::create()
          .table(Reactions::Table)
          .if_not_exists()
          .col(big_pk_auto(Reactions::Id))
          .col(big_integer(Reactions::SiteId))
          .col(string(Reactions::TargetType))
          .col(big_integer(Reactions::CommentId).null())
          .col(string(Reactions::PagePath))
          .col(string(Reactions::ActorType))
          .col(string(Reactions::ActorId))
          .col(string(Reactions::Type))
          .col(string(Reactions::TargetKey))
          .col(date_time(Reactions::CreatedAt))
          .index(
            Index::create()
              .name("idx-reactions-actor-target-unique")
              .table(Reactions::Table)
              .col(Reactions::ActorType)
              .col(Reactions::ActorId)
              .col(Reactions::TargetKey)
              .unique(),
          )
          .foreign_key(
            ForeignKey::create()
              .name("fk-reactions-site_id")
              .from(Reactions::Table, Reactions::SiteId)
              .to(Sites::Table, Sites::Id)
              .on_delete(ForeignKeyAction::Cascade)
              .on_update(ForeignKeyAction::Cascade),
          )
          .foreign_key(
            ForeignKey::create()
              .name("fk-reactions-comment_id")
              .from(Reactions::Table, Reactions::CommentId)
              .to(Comments::Table, Comments::Id)
              .on_delete(ForeignKeyAction::Cascade)
              .on_update(ForeignKeyAction::Cascade),
          )
          .to_owned(),
      )
      .await?;

    manager
      .create_table(
        Table::create()
          .table(CommentSubscriptions::Table)
          .if_not_exists()
          .col(big_pk_auto(CommentSubscriptions::Id))
          .col(big_integer(CommentSubscriptions::SiteId))
          .col(string(CommentSubscriptions::PagePath))
          .col(string(CommentSubscriptions::EventType))
          .col(big_integer(CommentSubscriptions::UserId).null())
          .col(string(CommentSubscriptions::Email).null())
          .col(boolean(CommentSubscriptions::IsActive).default(true))
          .col(date_time(CommentSubscriptions::CreatedAt))
          .col(date_time(CommentSubscriptions::UpdatedAt))
          .foreign_key(
            ForeignKey::create()
              .name("fk-comment-subscriptions-site_id")
              .from(CommentSubscriptions::Table, CommentSubscriptions::SiteId)
              .to(Sites::Table, Sites::Id)
              .on_delete(ForeignKeyAction::Cascade)
              .on_update(ForeignKeyAction::Cascade),
          )
          .foreign_key(
            ForeignKey::create()
              .name("fk-comment-subscriptions-user_id")
              .from(CommentSubscriptions::Table, CommentSubscriptions::UserId)
              .to(Users::Table, Users::Id)
              .on_delete(ForeignKeyAction::SetNull)
              .on_update(ForeignKeyAction::Cascade),
          )
          .to_owned(),
      )
      .await?;

    manager
      .create_table(
        Table::create()
          .table(Roles::Table)
          .if_not_exists()
          .col(big_pk_auto(Roles::Id))
          .col(string(Roles::Name))
          .col(string(Roles::Description).null())
          .col(date_time(Roles::CreatedAt))
          .col(date_time(Roles::UpdatedAt))
          .to_owned(),
      )
      .await?;

    manager
      .create_table(
        Table::create()
          .table(Permissions::Table)
          .if_not_exists()
          .col(big_pk_auto(Permissions::Id))
          .col(string(Permissions::Name))
          .col(string(Permissions::Description).null())
          .col(date_time(Permissions::CreatedAt))
          .col(date_time(Permissions::UpdatedAt))
          .to_owned(),
      )
      .await?;

    manager
      .create_table(
        Table::create()
          .table(RolePermissions::Table)
          .if_not_exists()
          .col(big_pk_auto(RolePermissions::Id))
          .col(big_integer(RolePermissions::RoleId))
          .col(big_integer(RolePermissions::PermissionId))
          .col(date_time(RolePermissions::CreatedAt))
          .col(date_time(RolePermissions::UpdatedAt))
          .foreign_key(
            ForeignKey::create()
              .name("fk-role-permissions-role_id")
              .from(RolePermissions::Table, RolePermissions::RoleId)
              .to(Roles::Table, Roles::Id)
              .on_delete(ForeignKeyAction::Cascade)
              .on_update(ForeignKeyAction::Cascade),
          )
          .foreign_key(
            ForeignKey::create()
              .name("fk-role-permissions-permission_id")
              .from(RolePermissions::Table, RolePermissions::PermissionId)
              .to(Permissions::Table, Permissions::Id)
              .on_delete(ForeignKeyAction::Cascade)
              .on_update(ForeignKeyAction::Cascade),
          )
          .to_owned(),
      )
      .await?;

    manager
      .create_table(
        Table::create()
          .table(UserRoleBindings::Table)
          .if_not_exists()
          .col(big_pk_auto(UserRoleBindings::Id))
          .col(big_integer(UserRoleBindings::UserId))
          .col(big_integer(UserRoleBindings::RoleId))
          .col(string(UserRoleBindings::ScopeType))
          .col(string(UserRoleBindings::ScopeId).null())
          .col(date_time(UserRoleBindings::CreatedAt))
          .col(date_time(UserRoleBindings::UpdatedAt))
          .foreign_key(
            ForeignKey::create()
              .name("fk-user-role-bindings-user_id")
              .from(UserRoleBindings::Table, UserRoleBindings::UserId)
              .to(Users::Table, Users::Id)
              .on_delete(ForeignKeyAction::Cascade)
              .on_update(ForeignKeyAction::Cascade),
          )
          .foreign_key(
            ForeignKey::create()
              .name("fk-user-role-bindings-role_id")
              .from(UserRoleBindings::Table, UserRoleBindings::RoleId)
              .to(Roles::Table, Roles::Id)
              .on_delete(ForeignKeyAction::Cascade)
              .on_update(ForeignKeyAction::Cascade),
          )
          .to_owned(),
      )
      .await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .drop_table(Table::drop().table(UserRoleBindings::Table).to_owned())
      .await?;
    manager
      .drop_table(Table::drop().table(RolePermissions::Table).to_owned())
      .await?;
    manager
      .drop_table(Table::drop().table(Permissions::Table).to_owned())
      .await?;
    manager
      .drop_table(Table::drop().table(Roles::Table).to_owned())
      .await?;
    manager
      .drop_table(Table::drop().table(CommentSubscriptions::Table).to_owned())
      .await?;
    manager
      .drop_table(Table::drop().table(Reactions::Table).to_owned())
      .await?;
    manager
      .drop_table(Table::drop().table(Comments::Table).to_owned())
      .await?;
    manager
      .drop_table(Table::drop().table(UserIdentities::Table).to_owned())
      .await?;
    manager
      .drop_table(Table::drop().table(ModerationProviders::Table).to_owned())
      .await?;
    manager
      .drop_table(Table::drop().table(OauthProviders::Table).to_owned())
      .await?;
    manager
      .drop_table(Table::drop().table(Sites::Table).to_owned())
      .await?;
    manager
      .drop_table(Table::drop().table(Users::Table).to_owned())
      .await
  }
}
