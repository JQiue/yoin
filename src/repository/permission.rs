use sea_orm::{
  ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, DbErr, EntityTrait,
  QueryFilter, entity::prelude::*,
};

use crate::entity::{
  permissions::{self},
  prelude::Permissions,
};

pub struct PermissionCreateData {
  pub name: String,
  pub description: Option<String>,
  pub datetime: DateTime,
}

#[derive(Default)]
pub struct PermissionUpdateData {
  pub id: i64,
  pub name: Option<String>,
  pub description: Option<Option<String>>,
  pub datetime: DateTime,
}

pub struct PermissionRepository {
  pub conn: &'static DatabaseConnection,
}

impl PermissionRepository {
  pub async fn find_all(&self) -> Result<Vec<permissions::Model>, DbErr> {
    Permissions::find().all(self.conn).await
  }

  pub async fn find_by_id(&self, id: i64) -> Result<Option<permissions::Model>, DbErr> {
    Permissions::find_by_id(id).one(self.conn).await
  }

  pub async fn find_by_name(&self, name: &str) -> Result<Option<permissions::Model>, DbErr> {
    Permissions::find()
      .filter(permissions::Column::Name.eq(name))
      .one(self.conn)
      .await
  }

  pub async fn create(&self, data: PermissionCreateData) -> Result<permissions::Model, DbErr> {
    permissions::ActiveModel {
      name: Set(data.name),
      description: Set(data.description),
      created_at: Set(data.datetime),
      updated_at: Set(data.datetime),
      ..Default::default()
    }
    .insert(self.conn)
    .await
  }

  pub async fn update(&self, data: PermissionUpdateData) -> Result<permissions::Model, DbErr> {
    let mut permission = permissions::ActiveModel {
      id: Set(data.id),
      updated_at: Set(data.datetime),
      ..Default::default()
    };

    if let Some(value) = data.name {
      permission.name = Set(value);
    }
    if let Some(value) = data.description {
      permission.description = Set(value);
    }

    permission.update(self.conn).await
  }
}
