use helpers::time::utc_now;
use migration::enums::{ReactionActorType, ReactionTargetType};
use sea_orm::{
  ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, DbErr, EntityTrait,
  entity::prelude::*,
};

use crate::entity::{prelude::Reactions, reactions};

pub struct ReactionCreateData {
  pub site_id: i64,
  pub target_type: ReactionTargetType,
  pub comment_id: Option<i64>,
  pub page_path: String,
  pub actor_type: ReactionActorType,
  pub actor_id: String,
  pub r#type: String,
  pub target_key: String,
}

pub struct ReactionRepository {
  pub conn: &'static DatabaseConnection,
}

impl ReactionRepository {
  pub async fn create(&self, data: ReactionCreateData) -> Result<reactions::Model, DbErr> {
    reactions::ActiveModel {
      site_id: Set(data.site_id),
      target_type: Set(data.target_type),
      comment_id: Set(data.comment_id),
      page_path: Set(data.page_path),
      actor_type: Set(data.actor_type),
      actor_id: Set(data.actor_id),
      r#type: Set(data.r#type),
      target_key: Set(data.target_key),
      created_at: Set(utc_now().naive_utc()),
      ..Default::default()
    }
    .insert(self.conn)
    .await
  }

  pub async fn find_by_actor_and_target(
    &self,
    actor_type: ReactionActorType,
    actor_id: &str,
    target_key: &str,
  ) -> Result<Option<reactions::Model>, DbErr> {
    Reactions::find()
      .filter(reactions::Column::ActorType.eq(actor_type))
      .filter(reactions::Column::ActorId.eq(actor_id))
      .filter(reactions::Column::TargetKey.eq(target_key))
      .one(self.conn)
      .await
  }

  pub async fn find_all_by_target_key(
    &self,
    target_key: &str,
  ) -> Result<Vec<reactions::Model>, DbErr> {
    Reactions::find()
      .filter(reactions::Column::TargetKey.eq(target_key))
      .all(self.conn)
      .await
  }

  pub async fn update_type(&self, id: i64, r#type: String) -> Result<reactions::Model, DbErr> {
    reactions::ActiveModel {
      id: Set(id),
      r#type: Set(r#type),
      ..Default::default()
    }
    .update(self.conn)
    .await
  }

  pub async fn delete_by_id(&self, id: i64) -> Result<u64, DbErr> {
    Reactions::delete_by_id(id)
      .exec(self.conn)
      .await
      .map(|res| res.rows_affected)
  }
}
