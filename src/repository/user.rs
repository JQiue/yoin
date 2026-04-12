use sea_orm::{
  ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, DbErr, EntityTrait,
  PaginatorTrait, QueryFilter, entity::prelude::*,
};

use crate::entity::{
  prelude::Users,
  users::{self},
};

pub struct UserCreateData {
  pub nickname: String,
  pub password: String,
  pub email: String,
  pub website: String,
  pub avatar: String,
  pub datetime: DateTime,
}

#[derive(Default)]
pub struct UserUpdateData {
  pub id: i64,
  pub nickname: Option<String>,
  pub password: Option<String>,
  pub email: Option<String>,
  pub website: Option<String>,
  pub avatar: Option<String>,
  pub datetime: DateTime,
}

pub struct UserRepository {
  pub conn: &'static DatabaseConnection,
}

impl UserRepository {
  pub async fn exists_any(&self) -> Result<bool, DbErr> {
    Ok(Users::find().count(self.conn).await? == 0)
  }

  pub async fn exists_by_email(&self, email: &str) -> Result<bool, DbErr> {
    let user = Users::find()
      .filter(users::Column::Email.eq(email))
      .one(self.conn)
      .await?;
    Ok(user.is_some())
  }

  pub async fn find_by_id(&self, user_id: i64) -> Result<Option<users::Model>, DbErr> {
    Users::find()
      .filter(users::Column::Id.eq(user_id))
      .one(self.conn)
      .await
  }

  pub async fn find_by_email(&self, email: &str) -> Result<Option<users::Model>, DbErr> {
    Users::find()
      .filter(users::Column::Email.eq(email))
      .one(self.conn)
      .await
  }

  pub async fn find_all(&self) -> Result<Vec<users::Model>, DbErr> {
    Users::find().all(self.conn).await
  }

  pub async fn create(&self, data: UserCreateData) -> Result<users::Model, DbErr> {
    let insert_user = users::ActiveModel {
      nickname: Set(data.nickname),
      password: Set(data.password),
      email: Set(data.email),
      avatar: Set(data.avatar),
      website: Set(data.website),
      created_at: Set(data.datetime),
      updated_at: Set(data.datetime),
      ..Default::default()
    };
    insert_user.insert(self.conn).await
  }

  pub async fn update(&self, data: UserUpdateData) -> Result<users::Model, DbErr> {
    let mut user = users::ActiveModel {
      id: Set(data.id),
      updated_at: Set(data.datetime),
      ..Default::default()
    };
    if let Some(nickname) = data.nickname {
      user.nickname = Set(nickname);
    }
    if let Some(password) = data.password {
      user.password = Set(password);
    }
    if let Some(email) = data.email {
      user.email = Set(email);
    }
    if let Some(website) = data.website {
      user.website = Set(website);
    }
    if let Some(avatar) = data.avatar {
      user.avatar = Set(avatar);
    }
    user.update(self.conn).await
  }
}
