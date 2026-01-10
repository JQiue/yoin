use axum::{
  Json,
  extract::{FromRequest, Request, State},
};
use serde::{Deserialize, Serialize};

use crate::{AppState, error::AppError, response::ApiResponse, service::create_user};

pub struct AppJson<T>(pub T);

impl<S, T> FromRequest<S> for AppJson<T>
where
  axum::Json<T>: FromRequest<S, Rejection = axum::extract::rejection::JsonRejection>,
  S: Send + Sync,
{
  type Rejection = AppError;
  async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
    match axum::Json::<T>::from_request(req, state).await {
      Ok(Json(value)) => Ok(AppJson(value)),
      Err(rejection) => match rejection {
        axum::extract::rejection::JsonRejection::MissingJsonContentType(
          missing_json_content_type,
        ) => Err(AppError::unsupported_media_type(
          missing_json_content_type.body_text(),
        )),
        other => Err(AppError::bad_request(other.body_text())),
      },
    }
  }
}

pub async fn health_check() -> Result<ApiResponse<()>, AppError> {
  Ok(ApiResponse::success(()))
}

#[derive(Deserialize)]
pub struct RegisterRequest {
  email: String,
  password: String,
  nickname: String,
  url: String,
}

#[derive(Serialize)]
pub struct RegisterResponse {
  pub avatar: String,
  pub nickname: String,
  pub url: String,
  pub token: String,
}

pub async fn register(
  State(state): State<AppState>,
  AppJson(payload): AppJson<RegisterRequest>,
) -> Result<ApiResponse<RegisterResponse>, AppError> {
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
