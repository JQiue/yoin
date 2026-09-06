use std::sync::Arc;

use axum::extract::{Path, State};
use migration::enums::ModerationProviderType;
use serde::{Deserialize, Serialize};

use crate::{
  app::AppState,
  entity::moderation_providers::ModerationProviderConfig,
  error::AppError,
  extractor::{AppJson, RequireAuth},
  rbac::permissions::codes::MODERATION_PROVIDER_MANAGE,
  response::ApiResponse,
};

#[derive(Debug, Serialize)]
pub struct ModerationProviderView {
  pub id: i64,
  pub site_id: i64,
  pub provider_kind: ModerationProviderType,
  pub enabled: bool,
  pub config: ModerationProviderConfig,
}

impl ModerationProviderView {
  pub fn from_model(model: crate::entity::moderation_providers::Model) -> Self {
    Self {
      id: model.id,
      site_id: model.site_id,
      provider_kind: model.provider_kind,
      enabled: model.enabled,
      config: model.config,
    }
  }
}

#[derive(Debug, Deserialize)]
pub struct CreateModerationProviderPayload {
  pub site_id: i64,
  pub provider_kind: ModerationProviderType,
  pub enabled: bool,
  pub config: ModerationProviderConfig,
}

#[derive(Debug, Deserialize)]
pub struct UpdateModerationProviderPayload {
  pub enabled: Option<bool>,
  pub config: Option<ModerationProviderConfig>,
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
    state
      .service
      .update_moderation_provider(id, payload)
      .await?,
  ))
}
