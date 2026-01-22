use std::sync::Arc;

use axum::extract::State;
use serde::{Deserialize, Serialize};

use crate::{AppState, error::AppError, extractor::AppJson, response::ApiResponse};

#[derive(Deserialize)]
pub struct RegisterPayload {
  email: String,
  password: String,
  nickname: String,
  url: String,
}

#[derive(Serialize)]
pub struct UserWithToken {
  pub token: String,
  #[serde(flatten)]
  pub user: UserProfile,
}

#[derive(Serialize)]
pub struct UserProfile {
  pub avatar: String,
  pub nickname: String,
  pub url: String,
  pub role: String,
}

pub async fn register(
  State(state): State<Arc<AppState>>,
  AppJson(payload): AppJson<RegisterPayload>,
) -> Result<ApiResponse<UserWithToken>, AppError> {
  Ok(ApiResponse::success(
    state
      .service
      .create_user(
        payload.nickname,
        payload.url,
        payload.email,
        payload.password,
        &state.jwt_key,
      )
      .await?,
  ))
}

#[derive(Deserialize)]
pub struct LoginPayload {
  pub email: String,
  pub password: String,
}

pub async fn login(
  State(state): State<Arc<AppState>>,
  AppJson(payload): AppJson<LoginPayload>,
) -> Result<ApiResponse<UserWithToken>, AppError> {
  Ok(ApiResponse::success(
    state.service.login_user(payload, &state.jwt_key).await?,
  ))
}
