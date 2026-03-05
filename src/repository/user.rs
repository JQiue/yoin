use sea_orm::{
  ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, Order,
  PaginatorTrait, QueryFilter, QueryOrder, entity::prelude::*,
};

use crate::entity::{
  prelude::{Comments, Sites, Users},
  sites::{self, SiteConfig},
  users::{self},
};

pub struct UserCreateData {
  pub nickname: String,
  pub password: String,
  pub email: String,
  pub website: String,
  pub avatar: String,
  pub role: migration::enums::UserRole,
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
  pub role: Option<migration::enums::UserRole>,
  pub datetime: DateTime,
}

pub trait UserRepositoryTrait {
  async fn exists_any(&self) -> Result<bool, DbErr>;
  async fn exists_by_email(&self, email: &str) -> Result<bool, DbErr>;
  async fn find_by_id(&self, user_id: i64) -> Result<Option<users::Model>, DbErr>;
  async fn find_by_email(&self, email: &str) -> Result<Option<users::Model>, DbErr>;
  async fn find_all(&self) -> Result<Vec<users::Model>, DbErr>;
  async fn create(&self, data: UserCreateData) -> Result<users::Model, DbErr>;
  async fn update(&self, data: UserUpdateData) -> Result<users::Model, DbErr>;
}

pub struct UserRepository {
  pub conn: &'static DatabaseConnection,
}

impl UserRepositoryTrait for UserRepository {
  async fn exists_any(&self) -> Result<bool, DbErr> {
    Ok(Users::find().count(self.conn).await? == 0)
  }

  async fn exists_by_email(&self, email: &str) -> Result<bool, DbErr> {
    let user = Users::find()
      .filter(users::Column::Email.eq(email))
      .one(self.conn)
      .await?;
    Ok(user.is_some())
  }

  async fn find_by_id(&self, user_id: i64) -> Result<Option<users::Model>, DbErr> {
    Users::find()
      .filter(users::Column::Id.eq(user_id))
      .one(self.conn)
      .await
  }

  async fn find_by_email(&self, email: &str) -> Result<Option<users::Model>, DbErr> {
    Users::find()
      .filter(users::Column::Email.eq(email))
      .one(self.conn)
      .await
  }

  async fn find_all(&self) -> Result<Vec<users::Model>, DbErr> {
    Users::find().all(self.conn).await
  }

  async fn create(&self, data: UserCreateData) -> Result<users::Model, DbErr> {
    let insert_user = users::ActiveModel {
      nickname: Set(data.nickname),
      password: Set(data.password),
      email: Set(data.email),
      avatar: Set(data.avatar),
      role: Set(data.role),
      website: Set(data.website),
      created_at: Set(data.datetime),
      updated_at: Set(data.datetime),
      ..Default::default()
    };
    insert_user.insert(self.conn).await
  }

  async fn update(&self, data: UserUpdateData) -> Result<users::Model, DbErr> {
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
    if let Some(role) = data.role {
      user.role = Set(role);
    }
    user.update(self.conn).await
  }
}
