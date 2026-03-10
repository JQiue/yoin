use sea_orm::{
  ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, DbErr, EntityTrait,
  QueryFilter, entity::prelude::*,
};

use crate::entity::{
  prelude::RolePermissions,
  role_permissions::{self},
};

pub struct RolePermissionCreateData {
  pub role_id: i64,
  pub permission_id: i64,
  pub datetime: DateTime,
}

pub struct RolePermissionRepository {
  pub conn: &'static DatabaseConnection,
}

impl RolePermissionRepository {
  pub async fn find_all(&self) -> Result<Vec<role_permissions::Model>, DbErr> {
    RolePermissions::find().all(self.conn).await
  }

  pub async fn find_all_by_role_id(
    &self,
    role_id: i64,
  ) -> Result<Vec<role_permissions::Model>, DbErr> {
    RolePermissions::find()
      .filter(role_permissions::Column::RoleId.eq(role_id))
      .all(self.conn)
      .await
  }

  pub async fn find_all_by_permission_id(
    &self,
    permission_id: i64,
  ) -> Result<Vec<role_permissions::Model>, DbErr> {
    RolePermissions::find()
      .filter(role_permissions::Column::PermissionId.eq(permission_id))
      .all(self.conn)
      .await
  }

  pub async fn find_by_role_and_permission(
    &self,
    role_id: i64,
    permission_id: i64,
  ) -> Result<Option<role_permissions::Model>, DbErr> {
    RolePermissions::find()
      .filter(role_permissions::Column::RoleId.eq(role_id))
      .filter(role_permissions::Column::PermissionId.eq(permission_id))
      .one(self.conn)
      .await
  }

  pub async fn create(
    &self,
    data: RolePermissionCreateData,
  ) -> Result<role_permissions::Model, DbErr> {
    role_permissions::ActiveModel {
      role_id: Set(data.role_id),
      permission_id: Set(data.permission_id),
      created_at: Set(data.datetime),
      updated_at: Set(data.datetime),
      ..Default::default()
    }
    .insert(self.conn)
    .await
  }
}
