use std::sync::Arc;

use axum::extract::{Path, State};
use serde::{Deserialize, Serialize};

use crate::{
  app::AppState,
  error::AppError,
  extractor::{AppJson, RequireAuth},
  response::ApiResponse,
};

#[derive(Debug, Deserialize)]
pub struct CreateReactionPayload {
  pub target_type: String,
  pub target_id: i64,
  pub reaction: String,
}

#[derive(Debug, Serialize)]
pub struct ReactionView {
  pub id: i64,
  pub target_type: String,
  pub target_id: i64,
  pub reaction: String,
}

pub async fn create(
  State(state): State<Arc<AppState>>,
  require_auth: RequireAuth,
  AppJson(payload): AppJson<CreateReactionPayload>,
) -> Result<ApiResponse<ReactionView>, AppError> {
  Ok(ApiResponse::success(
    state.service.create_reaction(require_auth.user_id, payload).await?,
  ))
}

pub async fn delete(
  State(state): State<Arc<AppState>>,
  require_auth: RequireAuth,
  Path(id): Path<i64>,
) -> Result<ApiResponse<()>, AppError> {
  state.service.delete_reaction(require_auth.user_id, id).await?;
  Ok(ApiResponse::success(()))
}
