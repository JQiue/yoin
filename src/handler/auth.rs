use axum::extract::State;
use serde::{Deserialize, Serialize};

use crate::{
  AppState,
  error::AppError,
  extractor::AppJson,
  response::ApiResponse,
  service::{create_user, login_user},
};

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
}

pub async fn register(
  State(state): State<AppState>,
  AppJson(payload): AppJson<RegisterPayload>,
) -> Result<ApiResponse<UserWithToken>, AppError> {
  Ok(ApiResponse::success(
    create_user(
      payload.nickname,
      payload.url,
      payload.email,
      payload.password,
      &state.conn,
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
  State(state): State<AppState>,
  AppJson(payload): AppJson<LoginPayload>,
) -> Result<ApiResponse<UserWithToken>, AppError> {
  Ok(ApiResponse::success(
    login_user(payload, &state.conn, &state.jwt_key).await?,
  ))
}
