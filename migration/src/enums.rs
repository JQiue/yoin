use sea_orm_migration::{
  prelude::*,
  sea_orm::{DeriveActiveEnum, EnumIter},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
#[serde(rename_all = "lowercase")]
pub enum CommentStatus {
  #[sea_orm(string_value = "pending")]
  Pending,
  #[sea_orm(string_value = "approved")]
  Approved,
  #[sea_orm(string_value = "spam")]
  Spam,
  #[sea_orm(string_value = "deleted")]
  Deleted,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum ReactionActorType {
  #[sea_orm(string_value = "user")]
  User,
  #[sea_orm(string_value = "guest")]
  Guest,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum ReactionTargetType {
  #[sea_orm(string_value = "comment")]
  Comment,
  #[sea_orm(string_value = "site")]
  Site,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum CommentSubscriptionEventType {
  #[sea_orm(string_value = "reply")]
  Reply,
  #[sea_orm(string_value = "mention")]
  Mention,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum UserRoleBindingScopeType {
  #[sea_orm(string_value = "global")]
  Global,
  #[sea_orm(string_value = "site")]
  Site,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
pub enum ModerationProviderType {
  #[sea_orm(string_value = "llm")]
  LLM,
  #[sea_orm(string_value = "akismet")]
  AKISMET,
}
