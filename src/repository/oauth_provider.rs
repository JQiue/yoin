use sea_orm::{
  ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, DbErr, EntityTrait,
  QueryFilter, entity::prelude::*,
};

use crate::entity::{
  oauth_providers::{self},
  prelude::OauthProviders,
};

pub struct OauthProviderCreateData {
  pub site_id: Option<i64>,
  pub provider_code: String,
  pub enabled: bool,
  pub client_id: String,
  pub client_secret: String,
  pub redirect_uri: String,
  pub datetime: DateTime,
}

#[derive(Default)]
pub struct OauthProviderUpdateData {
  pub id: i64,
  pub site_id: Option<Option<i64>>,
  pub provider_code: Option<String>,
  pub enabled: Option<bool>,
  pub client_id: Option<String>,
  pub client_secret: Option<String>,
  pub redirect_uri: Option<String>,
  pub datetime: DateTime,
}

pub struct OauthProviderRepository {
  pub conn: &'static DatabaseConnection,
}

impl OauthProviderRepository {
  pub async fn find_all(&self) -> Result<Vec<oauth_providers::Model>, DbErr> {
    OauthProviders::find().all(self.conn).await
  }

  pub async fn find_by_id(&self, id: i64) -> Result<Option<oauth_providers::Model>, DbErr> {
    OauthProviders::find_by_id(id).one(self.conn).await
  }

  pub async fn find_by_site_and_code(
    &self,
    site_id: Option<i64>,
    provider_code: &str,
  ) -> Result<Option<oauth_providers::Model>, DbErr> {
    let mut query =
      OauthProviders::find().filter(oauth_providers::Column::ProviderCode.eq(provider_code));

    query = match site_id {
      Some(site_id) => query.filter(oauth_providers::Column::SiteId.eq(site_id)),
      None => query.filter(oauth_providers::Column::SiteId.is_null()),
    };

    query.one(self.conn).await
  }

  pub async fn find_enabled_by_site_and_code(
    &self,
    site_id: Option<i64>,
    provider_code: &str,
  ) -> Result<Option<oauth_providers::Model>, DbErr> {
    let mut query = OauthProviders::find()
      .filter(oauth_providers::Column::ProviderCode.eq(provider_code))
      .filter(oauth_providers::Column::Enabled.eq(true));

    query = match site_id {
      Some(site_id) => query.filter(oauth_providers::Column::SiteId.eq(site_id)),
      None => query.filter(oauth_providers::Column::SiteId.is_null()),
    };

    query.one(self.conn).await
  }

  pub async fn create(
    &self,
    data: OauthProviderCreateData,
  ) -> Result<oauth_providers::Model, DbErr> {
    oauth_providers::ActiveModel {
      site_id: Set(data.site_id),
      provider_code: Set(data.provider_code),
      enabled: Set(data.enabled),
      client_id: Set(data.client_id),
      redirect_uri: Set(data.redirect_uri),
      client_secret: Set(data.client_secret),
      created_at: Set(data.datetime),
      updated_at: Set(data.datetime),
      ..Default::default()
    }
    .insert(self.conn)
    .await
  }

  pub async fn update(
    &self,
    data: OauthProviderUpdateData,
  ) -> Result<oauth_providers::Model, DbErr> {
    let mut provider = oauth_providers::ActiveModel {
      id: Set(data.id),
      updated_at: Set(data.datetime),
      ..Default::default()
    };

    if let Some(site_id) = data.site_id {
      provider.site_id = Set(site_id);
    }
    if let Some(provider_code) = data.provider_code {
      provider.provider_code = Set(provider_code);
    }
    if let Some(enabled) = data.enabled {
      provider.enabled = Set(enabled);
    }
    if let Some(client_id) = data.client_id {
      provider.client_id = Set(client_id);
    }
    if let Some(client_secret) = data.client_secret {
      provider.client_secret = Set(client_secret);
      if let Some(redirect_uri) = data.redirect_uri {
        provider.redirect_uri = Set(redirect_uri);
      }
    }

    provider.update(self.conn).await
  }
}
