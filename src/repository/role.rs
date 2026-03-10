use sea_orm::{
  ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, DbErr, EntityTrait,
  QueryFilter, entity::prelude::*,
};

use crate::entity::{
  prelude::Roles,
  roles::{self},
};

pub struct RoleCreateData {
  pub name: String,
  pub description: Option<String>,
  pub datetime: DateTime,
}

#[derive(Default)]
pub struct RoleUpdateData {
  pub id: i64,
  pub name: Option<String>,
  pub description: Option<Option<String>>,
  pub datetime: DateTime,
}

pub struct RoleRepository {
  pub conn: &'static DatabaseConnection,
}

impl RoleRepository {
  pub async fn find_all(&self) -> Result<Vec<roles::Model>, DbErr> {
    Roles::find().all(self.conn).await
  }

  pub async fn find_by_id(&self, id: i64) -> Result<Option<roles::Model>, DbErr> {
    Roles::find_by_id(id).one(self.conn).await
  }

  pub async fn find_by_name(&self, name: &str) -> Result<Option<roles::Model>, DbErr> {
    Roles::find()
      .filter(roles::Column::Name.eq(name))
      .one(self.conn)
      .await
  }

  pub async fn create(&self, data: RoleCreateData) -> Result<roles::Model, DbErr> {
    roles::ActiveModel {
      name: Set(data.name),
      description: Set(data.description),
      created_at: Set(data.datetime),
      updated_at: Set(data.datetime),
      ..Default::default()
    }
    .insert(self.conn)
    .await
  }

  pub async fn update(&self, data: RoleUpdateData) -> Result<roles::Model, DbErr> {
    let mut role = roles::ActiveModel {
      id: Set(data.id),
      updated_at: Set(data.datetime),
      ..Default::default()
    };

    if let Some(value) = data.name {
      role.name = Set(value);
    }
    if let Some(value) = data.description {
      role.description = Set(value);
    }

    role.update(self.conn).await
  }
}
