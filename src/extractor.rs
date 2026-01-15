use axum::{
  Json,
  extract::{FromRequest, Request, State},
};

use crate::error::AppError;

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

use axum::{
  extract::FromRequestParts,
  http::{StatusCode, header::AUTHORIZATION, request::Parts},
};
use helpers::jwt;

use crate::AppState;

pub struct RequireAuth {
  pub user_id: i64,
}

impl FromRequestParts<AppState> for RequireAuth {
  type Rejection = StatusCode;

  async fn from_request_parts(
    parts: &mut Parts,
    state: &AppState,
  ) -> Result<Self, Self::Rejection> {
    let auth_header = parts
      .headers
      .get(AUTHORIZATION)
      .and_then(|value| value.to_str().ok());

    if let Some(auth_header) = auth_header
      && auth_header.starts_with("Bearer ")
    {
      let token = auth_header.trim_start_matches("Bearer ");
      if let Ok(data) = jwt::verify(token, &state.jwt_key) {
        Ok(RequireAuth {
          user_id: data.claims.data,
        })
      } else {
        Err(StatusCode::UNAUTHORIZED)
      }
    } else {
      Err(StatusCode::UNAUTHORIZED)
    }
  }
}

#[derive(Debug)]
pub struct OptionnalAuth {
  pub user_id: Option<i64>,
}

impl FromRequestParts<AppState> for OptionnalAuth {
  type Rejection = StatusCode;

  async fn from_request_parts(
    parts: &mut Parts,
    state: &AppState,
  ) -> Result<Self, Self::Rejection> {
    let auth_header = parts
      .headers
      .get(AUTHORIZATION)
      .and_then(|value| value.to_str().ok());

    if let Some(auth_header) = auth_header
      && auth_header.starts_with("Bearer ")
    {
      let token = auth_header.trim_start_matches("Bearer ");
      if let Ok(data) = jwt::verify(token, &state.jwt_key) {
        Ok(OptionnalAuth {
          user_id: Some(data.claims.data),
        })
      } else {
        Err(StatusCode::UNAUTHORIZED)
      }
    } else {
      Ok(OptionnalAuth { user_id: None })
    }
  }
}
