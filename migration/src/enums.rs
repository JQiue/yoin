use std::fmt::Display;

use sea_orm_migration::{
  prelude::*,
  sea_orm::{DeriveActiveEnum, EnumIter},
};

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
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
pub enum UserRole {
  #[sea_orm(string_value = "normal")]
  Normal,
  #[sea_orm(string_value = "admin")]
  Admin,
}

impl Display for UserRole {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      UserRole::Normal => write!(f, "normal"),
      UserRole::Admin => write!(f, "admin"),
    }
  }
}
