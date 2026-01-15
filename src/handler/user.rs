use axum::extract::State;
use serde::Deserialize;

use crate::{
  AppState,
  error::AppError,
  extractor::{AppJson, RequireAuth},
  handler::auth::UserProfile,
  response::ApiResponse,
  service::{fetch_profile, update_user_profile},
};

pub async fn profile(
  State(state): State<AppState>,
  require_auth: RequireAuth,
) -> Result<ApiResponse<UserProfile>, AppError> {
  Ok(ApiResponse::success(
    fetch_profile(require_auth.user_id, &state.conn).await?,
  ))
}

#[derive(Deserialize)]
pub struct UpdateProfilePayload {
  pub nickname: Option<String>,
  pub avatar: Option<String>,
  pub url: Option<String>,
}

pub async fn update_profile(
  State(state): State<AppState>,
  require_auth: RequireAuth,
  AppJson(payload): AppJson<UpdateProfilePayload>,
) -> Result<ApiResponse<UserProfile>, AppError> {
  Ok(ApiResponse::success(
    update_user_profile(require_auth.user_id, payload, &state.conn).await?,
  ))
}
