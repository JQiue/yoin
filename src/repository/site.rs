use sea_orm::{
  ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, DbErr, EntityTrait,
  entity::prelude::*,
};

use crate::entity::{
  prelude::Sites,
  sites::{self, SiteConfig},
};

pub struct SiteCreateData {
  pub name: String,
  pub url: String,
  pub config: SiteConfig,
  pub datetime: DateTime,
}

pub struct SiteUpdateData {
  pub id: i64,
  pub name: Option<String>,
  pub url: Option<String>,
  pub config: Option<SiteConfig>,
  pub datetime: DateTime,
}

pub trait SiteRepositoryTrait {
  async fn create_default(&self, datetime: DateTime) -> Result<sites::Model, DbErr>;
  async fn create(&self, data: SiteCreateData) -> Result<sites::Model, DbErr>;
  async fn update(&self, data: SiteUpdateData) -> Result<sites::Model, DbErr>;
  async fn find_all(&self) -> Result<Vec<sites::Model>, DbErr>;
  async fn find_by_id(&self, id: i64) -> Result<Option<sites::Model>, DbErr>;
}

pub struct SiteRepository {
  pub conn: &'static DatabaseConnection,
}

impl SiteRepositoryTrait for SiteRepository {
  async fn find_all(&self) -> Result<Vec<sites::Model>, DbErr> {
    Sites::find().all(self.conn).await
  }

  async fn find_by_id(&self, id: i64) -> Result<Option<sites::Model>, DbErr> {
    Sites::find()
      .filter(sites::Column::Id.eq(id))
      .one(self.conn)
      .await
  }

  async fn create_default(&self, datetime: DateTime) -> Result<sites::Model, DbErr> {
    let new_site = sites::ActiveModel {
      name: Set("Default Site".to_string()),
      url: Set("".to_string()),
      config: Set(SiteConfig::default()),
      created_at: Set(datetime),
      updated_at: Set(datetime),
      ..Default::default()
    };
    new_site.insert(self.conn).await
  }

  async fn create(&self, data: SiteCreateData) -> Result<sites::Model, DbErr> {
    let new_site = sites::ActiveModel {
      name: Set(data.name),
      url: Set(data.url),
      config: Set(data.config),
      created_at: Set(data.datetime),
      updated_at: Set(data.datetime),
      ..Default::default()
    };
    new_site.insert(self.conn).await
  }

  async fn update(&self, data: SiteUpdateData) -> Result<sites::Model, DbErr> {
    let mut site = sites::ActiveModel {
      id: Set(data.id),
      updated_at: Set(data.datetime),
      ..Default::default()
    };

    if let Some(name) = data.name {
      site.name = Set(name);
    }

    if let Some(url) = data.url {
      site.url = Set(url);
    }

    if let Some(config) = data.config {
      site.config = Set(config);
    }

    site.update(self.conn).await
  }
}
