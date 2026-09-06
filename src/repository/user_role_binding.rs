use migration::enums::UserRoleBindingScopeType;
use sea_orm::{
  ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, DbErr, EntityTrait,
  QueryFilter, entity::prelude::*,
};

use crate::entity::{
  prelude::UserRoleBindings,
  user_role_bindings::{self},
};

pub struct UserRoleBindingCreateData {
  pub user_id: i64,
  pub role_id: i64,
  pub scope_type: UserRoleBindingScopeType,
  pub scope_id: Option<String>,
  pub datetime: DateTime,
}

#[derive(Default)]
pub struct UserRoleBindingUpdateData {
  pub id: i64,
  pub role_id: Option<i64>,
  pub scope_type: Option<UserRoleBindingScopeType>,
  pub scope_id: Option<Option<String>>,
  pub datetime: DateTime,
}

pub struct UserRoleBindingRepository {
  pub conn: &'static DatabaseConnection,
}

impl UserRoleBindingRepository {
  pub async fn find_all(&self) -> Result<Vec<user_role_bindings::Model>, DbErr> {
    UserRoleBindings::find().all(self.conn).await
  }

  pub async fn find_by_id(&self, id: i64) -> Result<Option<user_role_bindings::Model>, DbErr> {
    UserRoleBindings::find_by_id(id).one(self.conn).await
  }

  pub async fn find_all_by_user_id(
    &self,
    user_id: i64,
  ) -> Result<Vec<user_role_bindings::Model>, DbErr> {
    UserRoleBindings::find()
      .filter(user_role_bindings::Column::UserId.eq(user_id))
      .all(self.conn)
      .await
  }

  pub async fn find_all_by_user_and_scope(
    &self,
    user_id: i64,
    scope_type: UserRoleBindingScopeType,
    scope_id: Option<&str>,
  ) -> Result<Vec<user_role_bindings::Model>, DbErr> {
    let mut query = UserRoleBindings::find()
      .filter(user_role_bindings::Column::UserId.eq(user_id))
      .filter(user_role_bindings::Column::ScopeType.eq(scope_type));

    query = match scope_id {
      Some(scope_id) => query.filter(user_role_bindings::Column::ScopeId.eq(scope_id)),
      None => query.filter(user_role_bindings::Column::ScopeId.is_null()),
    };

    query.all(self.conn).await
  }

  pub async fn find_all_by_role_and_scope(
    &self,
    role_id: i64,
    scope_type: UserRoleBindingScopeType,
    scope_id: Option<&str>,
  ) -> Result<Vec<user_role_bindings::Model>, DbErr> {
    let mut query = UserRoleBindings::find()
      .filter(user_role_bindings::Column::RoleId.eq(role_id))
      .filter(user_role_bindings::Column::ScopeType.eq(scope_type));

    query = match scope_id {
      Some(scope_id) => query.filter(user_role_bindings::Column::ScopeId.eq(scope_id)),
      None => query.filter(user_role_bindings::Column::ScopeId.is_null()),
    };

    query.all(self.conn).await
  }

  pub async fn create(
    &self,
    data: UserRoleBindingCreateData,
  ) -> Result<user_role_bindings::Model, DbErr> {
    user_role_bindings::ActiveModel {
      user_id: Set(data.user_id),
      role_id: Set(data.role_id),
      scope_type: Set(data.scope_type),
      scope_id: Set(data.scope_id),
      created_at: Set(data.datetime),
      updated_at: Set(data.datetime),
      ..Default::default()
    }
    .insert(self.conn)
    .await
  }

  pub async fn update(
    &self,
    data: UserRoleBindingUpdateData,
  ) -> Result<user_role_bindings::Model, DbErr> {
    let mut binding = user_role_bindings::ActiveModel {
      id: Set(data.id),
      updated_at: Set(data.datetime),
      ..Default::default()
    };

    if let Some(value) = data.role_id {
      binding.role_id = Set(value);
    }
    if let Some(value) = data.scope_type {
      binding.scope_type = Set(value);
    }
    if let Some(value) = data.scope_id {
      binding.scope_id = Set(value);
    }

    binding.update(self.conn).await
  }

  pub async fn delete_by_id(&self, id: i64) -> Result<u64, DbErr> {
    let result = UserRoleBindings::delete_by_id(id).exec(self.conn).await?;
    Ok(result.rows_affected)
  }
}
