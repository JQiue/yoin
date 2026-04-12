use migration::enums::ModerationProviderType;
use sea_orm::{
  ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, DbErr, EntityTrait,
  QueryFilter, entity::prelude::*,
};

use crate::entity::{
  moderation_providers::{self, ModerationProviderConfig},
  prelude::ModerationProviders,
};

pub struct ModerationProviderCreateData {
  pub site_id: i64,
  pub provider_kind: ModerationProviderType,
  pub enabled: bool,
  pub config: ModerationProviderConfig,
  pub datetime: DateTime,
}

#[derive(Default)]
pub struct ModerationProviderUpdateData {
  pub id: i64,
  pub enabled: Option<bool>,
  pub config: Option<ModerationProviderConfig>,
  pub datetime: DateTime,
}

pub struct ModerationProviderRepository {
  pub conn: &'static DatabaseConnection,
}

impl ModerationProviderRepository {
  pub async fn find_all(&self) -> Result<Vec<moderation_providers::Model>, DbErr> {
    ModerationProviders::find().all(self.conn).await
  }

  pub async fn find_by_id(&self, id: i64) -> Result<Option<moderation_providers::Model>, DbErr> {
    ModerationProviders::find_by_id(id).one(self.conn).await
  }

  pub async fn find_by_site_and_provider(
    &self,
    site_id: i64,
    provider_kind: &str,
  ) -> Result<Option<moderation_providers::Model>, DbErr> {
    ModerationProviders::find()
      .filter(moderation_providers::Column::SiteId.eq(site_id))
      .filter(moderation_providers::Column::ProviderKind.eq(provider_kind))
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
      provider_kind: Set(data.provider_kind),
      enabled: Set(data.enabled),
      config: Set(data.config),
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

    if let Some(value) = data.enabled {
      provider.enabled = Set(value);
    }

    if let Some(value) = data.config {
      provider.config = Set(value);
    }

    provider.update(self.conn).await
  }
}
