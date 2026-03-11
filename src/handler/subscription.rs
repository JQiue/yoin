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
pub struct CreateCommentSubscriptionPayload {
  pub site_id: i64,
  pub page_path: String,
  pub event_type: String,
}

#[derive(Debug, Serialize)]
pub struct CommentSubscriptionView {
  pub id: i64,
  pub site_id: i64,
  pub page_path: String,
  pub event_type: String,
}

pub async fn list(
  State(state): State<Arc<AppState>>,
  require_auth: RequireAuth,
) -> Result<ApiResponse<Vec<CommentSubscriptionView>>, AppError> {
  Ok(ApiResponse::success(
    state
      .service
      .list_comment_subscriptions(require_auth.user_id)
      .await?,
  ))
}

pub async fn create(
  State(state): State<Arc<AppState>>,
  require_auth: RequireAuth,
  AppJson(payload): AppJson<CreateCommentSubscriptionPayload>,
) -> Result<ApiResponse<CommentSubscriptionView>, AppError> {
  Ok(ApiResponse::success(
    state
      .service
      .create_comment_subscription(require_auth.user_id, payload)
      .await?,
  ))
}

pub async fn delete(
  State(state): State<Arc<AppState>>,
  require_auth: RequireAuth,
  Path(id): Path<i64>,
) -> Result<ApiResponse<()>, AppError> {
  state
    .service
    .delete_comment_subscription(require_auth.user_id, id)
    .await?;
  Ok(ApiResponse::success(()))
}
