use sea_orm::{
  ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, DbErr, EntityTrait,
  QueryFilter, entity::prelude::*,
};

use crate::entity::{
  prelude::UserIdentities,
  user_identities::{self},
};

pub struct UserIdentityCreateData {
  pub user_id: i64,
  pub provider: String,
  pub provider_user_id: String,
  pub email: Option<String>,
  pub profile: Option<Json>,
  pub datetime: DateTime,
}

#[derive(Default)]
pub struct UserIdentityUpdateData {
  pub id: i64,
  pub email: Option<Option<String>>,
  pub profile: Option<Option<Json>>,
  pub datetime: DateTime,
}

pub struct UserIdentityRepository {
  pub conn: &'static DatabaseConnection,
}

impl UserIdentityRepository {
  pub async fn find_by_id(&self, id: i64) -> Result<Option<user_identities::Model>, DbErr> {
    UserIdentities::find_by_id(id).one(self.conn).await
  }

  pub async fn find_by_provider_subject(
    &self,
    provider: &str,
    provider_user_id: &str,
  ) -> Result<Option<user_identities::Model>, DbErr> {
    UserIdentities::find()
      .filter(user_identities::Column::Provider.eq(provider))
      .filter(user_identities::Column::ProviderUserId.eq(provider_user_id))
      .one(self.conn)
      .await
  }

  pub async fn find_all_by_user_id(
    &self,
    user_id: i64,
  ) -> Result<Vec<user_identities::Model>, DbErr> {
    UserIdentities::find()
      .filter(user_identities::Column::UserId.eq(user_id))
      .all(self.conn)
      .await
  }

  pub async fn create(
    &self,
    data: UserIdentityCreateData,
  ) -> Result<user_identities::Model, DbErr> {
    user_identities::ActiveModel {
      user_id: Set(data.user_id),
      provider: Set(data.provider),
      provider_user_id: Set(data.provider_user_id),
      email: Set(data.email),
      profile: Set(data.profile),
      created_at: Set(data.datetime),
      updated_at: Set(data.datetime),
      ..Default::default()
    }
    .insert(self.conn)
    .await
  }

  pub async fn update(
    &self,
    data: UserIdentityUpdateData,
  ) -> Result<user_identities::Model, DbErr> {
    let mut identity = user_identities::ActiveModel {
      id: Set(data.id),
      updated_at: Set(data.datetime),
      ..Default::default()
    };

    if let Some(value) = data.email {
      identity.email = Set(value);
    }
    if let Some(value) = data.profile {
      identity.profile = Set(value);
    }

    identity.update(self.conn).await
  }
}
