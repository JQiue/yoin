use std::sync::Arc;

use axum::extract::{Path, State};
use serde::{Deserialize, Serialize};

use crate::{
  app::AppState,
  error::AppError,
  extractor::{AppJson, RequireAuth},
  handler::comment::CommentView,
  rbac::permissions::codes::{COMMENT_MODERATE, MODERATION_PROVIDER_MANAGE},
  response::ApiResponse,
};

#[derive(Debug, Serialize)]
pub struct ModerationProviderView {
  pub id: i64,
  pub provider: String,
  pub enabled: bool,
  pub model: Option<String>,
  pub api_base: Option<String>,
  pub prompt: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateModerationProviderPayload {
  pub site_id: i64,
  pub provider: String,
  pub enabled: bool,
  pub model: Option<String>,
  pub api_base: Option<String>,
  pub api_key: Option<String>,
  pub prompt: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateModerationProviderPayload {
  pub enabled: Option<bool>,
  pub model: Option<String>,
  pub api_base: Option<String>,
  pub api_key: Option<String>,
  pub prompt: Option<String>,
}

pub async fn list_providers(
  State(state): State<Arc<AppState>>,
  require_auth: RequireAuth,
) -> Result<ApiResponse<Vec<ModerationProviderView>>, AppError> {
  state
    .service
    .require_global_permission(require_auth.user_id, MODERATION_PROVIDER_MANAGE)
    .await?;
  Ok(ApiResponse::success(
    state.service.list_moderation_providers().await?,
  ))
}

pub async fn create_provider(
  State(state): State<Arc<AppState>>,
  require_auth: RequireAuth,
  AppJson(payload): AppJson<CreateModerationProviderPayload>,
) -> Result<ApiResponse<ModerationProviderView>, AppError> {
  state
    .service
    .require_global_permission(require_auth.user_id, MODERATION_PROVIDER_MANAGE)
    .await?;
  Ok(ApiResponse::success(
    state.service.create_moderation_provider(payload).await?,
  ))
}

pub async fn update_provider(
  State(state): State<Arc<AppState>>,
  require_auth: RequireAuth,
  Path(id): Path<i64>,
  AppJson(payload): AppJson<UpdateModerationProviderPayload>,
) -> Result<ApiResponse<ModerationProviderView>, AppError> {
  state
    .service
    .require_global_permission(require_auth.user_id, MODERATION_PROVIDER_MANAGE)
    .await?;
  Ok(ApiResponse::success(
    state.service.update_moderation_provider(id, payload).await?,
  ))
}

pub async fn list_pending_comments(
  State(state): State<Arc<AppState>>,
  require_auth: RequireAuth,
) -> Result<ApiResponse<Vec<CommentView>>, AppError> {
  state
    .service
    .require_global_permission(require_auth.user_id, COMMENT_MODERATE)
    .await?;
  Ok(ApiResponse::success(
    state.service.list_pending_comments().await?,
  ))
}

pub async fn approve_comment(
  State(state): State<Arc<AppState>>,
  require_auth: RequireAuth,
  Path(id): Path<i64>,
) -> Result<ApiResponse<()>, AppError> {
  state
    .service
    .require_global_permission(require_auth.user_id, COMMENT_MODERATE)
    .await?;
  state.service.approve_comment(id).await?;
  Ok(ApiResponse::success(()))
}

pub async fn reject_comment(
  State(state): State<Arc<AppState>>,
  require_auth: RequireAuth,
  Path(id): Path<i64>,
) -> Result<ApiResponse<()>, AppError> {
  state
    .service
    .require_global_permission(require_auth.user_id, COMMENT_MODERATE)
    .await?;
  state.service.reject_comment(id).await?;
  Ok(ApiResponse::success(()))
}
