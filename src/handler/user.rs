use std::sync::Arc;

use axum::extract::State;
use serde::Deserialize;

use crate::{
  AppState,
  error::AppError,
  extractor::{AppJson, RequireAuth},
  handler::auth::UserProfile,
  response::ApiResponse,
};

pub async fn profile(
  State(state): State<Arc<AppState>>,
  require_auth: RequireAuth,
) -> Result<ApiResponse<UserProfile>, AppError> {
  Ok(ApiResponse::success(
    state.service.fetch_profile(require_auth.user_id).await?,
  ))
}

#[derive(Deserialize)]
pub struct UpdateProfilePayload {
  pub nickname: Option<String>,
  pub avatar: Option<String>,
  pub website: Option<String>,
}

pub async fn update_profile(
  State(state): State<Arc<AppState>>,
  require_auth: RequireAuth,
  AppJson(payload): AppJson<UpdateProfilePayload>,
) -> Result<ApiResponse<UserProfile>, AppError> {
  Ok(ApiResponse::success(
    state
      .service
      .update_user_profile(require_auth.user_id, payload)
      .await?,
  ))
}
