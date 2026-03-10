use sea_orm::{
  ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, DbErr, EntityTrait,
  QueryFilter, entity::prelude::*,
};

use crate::entity::{
  moderation_providers::{self},
  prelude::ModerationProviders,
};

pub struct ModerationProviderCreateData {
  pub site_id: i64,
  pub provider: String,
  pub enabled: bool,
  pub model: String,
  pub api_base: String,
  pub api_key: String,
  pub prompt: Option<String>,
  pub datetime: DateTime,
}

#[derive(Default)]
pub struct ModerationProviderUpdateData {
  pub id: i64,
  pub provider: Option<String>,
  pub enabled: Option<bool>,
  pub model: Option<String>,
  pub api_base: Option<String>,
  pub api_key: Option<String>,
  pub prompt: Option<Option<String>>,
  pub datetime: DateTime,
}

pub struct ModerationProviderRepository {
  pub conn: &'static DatabaseConnection,
}

impl ModerationProviderRepository {
  pub async fn find_all(&self) -> Result<Vec<moderation_providers::Model>, DbErr> {
    ModerationProviders::find().all(self.conn).await
  }

  pub async fn find_by_id(
    &self,
    id: i64,
  ) -> Result<Option<moderation_providers::Model>, DbErr> {
    ModerationProviders::find_by_id(id).one(self.conn).await
  }

  pub async fn find_by_site_and_provider(
    &self,
    site_id: i64,
    provider: &str,
  ) -> Result<Option<moderation_providers::Model>, DbErr> {
    ModerationProviders::find()
      .filter(moderation_providers::Column::SiteId.eq(site_id))
      .filter(moderation_providers::Column::Provider.eq(provider))
      .one(self.conn)
      .await
  }

  pub async fn find_enabled_by_site(
    &self,
    site_id: i64,
  ) -> Result<Option<moderation_providers::Model>, DbErr> {
    ModerationProviders::find()
      .filter(moderation_providers::Column::SiteId.eq(site_id))
      .filter(moderation_providers::Column::Enabled.eq(true))
      .one(self.conn)
      .await
  }

  pub async fn create(
    &self,
    data: ModerationProviderCreateData,
  ) -> Result<moderation_providers::Model, DbErr> {
    moderation_providers::ActiveModel {
      site_id: Set(data.site_id),
      provider: Set(data.provider),
      enabled: Set(data.enabled),
      model: Set(data.model),
      api_base: Set(data.api_base),
      api_key: Set(data.api_key),
      prompt: Set(data.prompt),
      created_at: Set(data.datetime),
      updated_at: Set(data.datetime),
      ..Default::default()
    }
    .insert(self.conn)
    .await
  }

  pub async fn update(
    &self,
    data: ModerationProviderUpdateData,
  ) -> Result<moderation_providers::Model, DbErr> {
    let mut provider = moderation_providers::ActiveModel {
      id: Set(data.id),
      updated_at: Set(data.datetime),
      ..Default::default()
    };

    if let Some(value) = data.provider {
      provider.provider = Set(value);
    }
    if let Some(value) = data.enabled {
      provider.enabled = Set(value);
    }
    if let Some(value) = data.model {
      provider.model = Set(value);
    }
    if let Some(value) = data.api_base {
      provider.api_base = Set(value);
    }
    if let Some(value) = data.api_key {
      provider.api_key = Set(value);
    }
    if let Some(value) = data.prompt {
      provider.prompt = Set(value);
    }

    provider.update(self.conn).await
  }
}
