use std::sync::Arc;

use axum::extract::{Query, State};
use serde::{Deserialize, Serialize};

use crate::{
  app::AppState,
  error::AppError,
  extractor::{AppJson, OptionnalAuth},
  response::ApiResponse,
};

#[derive(Debug, Deserialize)]
pub struct UpsertReactionPayload {
  pub site_id: i64,
  pub target_type: String,
  pub comment_id: Option<i64>,
  pub page_path: String,
  pub reaction: String,
}

#[derive(Debug, Deserialize)]
pub struct ListReactionsQuery {
  pub site_id: i64,
  pub target_type: String,
  pub comment_id: Option<i64>,
  pub page_path: String,
}

#[derive(Debug, Serialize)]
pub struct ReactionSummaryView {
  pub counts: std::collections::BTreeMap<String, u64>,
  pub my_reaction: Option<String>,
}

pub async fn upsert(
  State(state): State<Arc<AppState>>,
  optional_auth: OptionnalAuth,
  AppJson(payload): AppJson<UpsertReactionPayload>,
) -> Result<ApiResponse<ReactionSummaryView>, AppError> {
  Ok(ApiResponse::success(
    state
      .service
      .upsert_reaction(optional_auth.user_id, payload)
      .await?,
  ))
}

pub async fn list(
  State(state): State<Arc<AppState>>,
  optional_auth: OptionnalAuth,
  Query(qs): Query<ListReactionsQuery>,
) -> Result<ApiResponse<ReactionSummaryView>, AppError> {
  Ok(ApiResponse::success(
    state
      .service
      .list_reactions(optional_auth.user_id, qs)
      .await?,
  ))
}
