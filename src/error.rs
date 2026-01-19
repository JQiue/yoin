use axum::{
  Json,
  http::StatusCode,
  response::{IntoResponse, Response},
};
use thiserror::Error;

use crate::response::ApiResponse;

#[derive(Debug, Error)]
pub enum AppError {
  #[error("{msg}")]
  Client { kind: ClientErrorKind, msg: String },
  #[error("Internal error: {msg}")]
  Internal {
    msg: String,
    source: Option<Box<dyn std::error::Error + Send + Sync>>,
  },
}

#[derive(Debug)]
pub enum ClientErrorKind {
  BadRequest(i32),
  Unauthorized(i32),
  Forbidden(i32),
  NotFound(i32),
  Conflict(i32),
  UnsupportedMediaType(i32),
  RateLimited(i32),
}

impl ClientErrorKind {
  pub const COMMON_MODULE: i32 = 0;
  pub const AUTH_MODULE: i32 = 100_000;
  pub const USER_MODULE: i32 = 200_000;
  pub const SITE_MODULE: i32 = 300_000;

  pub fn bad_request() -> Self {
    Self::BadRequest(Self::COMMON_MODULE + 400)
  }

  pub fn unsupported_media_type() -> Self {
    Self::UnsupportedMediaType(Self::COMMON_MODULE + 415)
  }

  pub fn invalid_credentials() -> Self {
    Self::Unauthorized(Self::AUTH_MODULE + 401)
  }

  pub fn user_not_found() -> Self {
    Self::NotFound(Self::USER_MODULE + 404)
  }

  pub fn site_not_found() -> Self {
    Self::NotFound(Self::SITE_MODULE + 404)
  }

  pub fn user_already_exists() -> Self {
    Self::Conflict(Self::USER_MODULE + 409)
  }
}

impl AppError {
  pub fn bad_request(msg: String) -> Self {
    Self::Client {
      kind: ClientErrorKind::bad_request(),
      msg,
    }
  }

  pub fn forbidden(msg: String) -> Self {
    Self::Client {
      kind: ClientErrorKind::Forbidden(ClientErrorKind::AUTH_MODULE + 403),
      msg,
    }
  }

  pub fn invalid_credentials(msg: String) -> Self {
    Self::Client {
      kind: ClientErrorKind::invalid_credentials(),
      msg,
    }
  }

  pub fn unsupported_media_type(msg: String) -> Self {
    Self::Client {
      kind: ClientErrorKind::unsupported_media_type(),
      msg,
    }
  }

  pub fn user_not_found(msg: String) -> Self {
    Self::Client {
      kind: ClientErrorKind::user_not_found(),
      msg,
    }
  }

  pub fn user_already_exists(msg: String) -> Self {
    Self::Client {
      kind: ClientErrorKind::user_already_exists(),
      msg,
    }
  }

  pub fn site_not_found(msg: String) -> Self {
    Self::Client {
      kind: ClientErrorKind::site_not_found(),
      msg,
    }
  }
}

impl IntoResponse for AppError {
  fn into_response(self) -> Response {
    let (status_code, business_code, msg) = match self {
      AppError::Client { kind, msg } => {
        let (status_code, biz_code) = match kind {
          ClientErrorKind::BadRequest(code) => (StatusCode::BAD_REQUEST, code),
          ClientErrorKind::Unauthorized(code) => (StatusCode::UNAUTHORIZED, code),
          ClientErrorKind::Forbidden(code) => (StatusCode::FORBIDDEN, code),
          ClientErrorKind::NotFound(code) => (StatusCode::NOT_FOUND, code),
          ClientErrorKind::Conflict(code) => (StatusCode::CONFLICT, code),
          ClientErrorKind::RateLimited(code) => (StatusCode::TOO_MANY_REQUESTS, code),
          ClientErrorKind::UnsupportedMediaType(code) => (StatusCode::UNSUPPORTED_MEDIA_TYPE, code),
        };
        (status_code, biz_code, msg)
      }
      AppError::Internal { .. } => (
        StatusCode::INTERNAL_SERVER_ERROR,
        500_000,
        "An internal server error occurred".to_string(),
      ),
    };
    let resp = ApiResponse::<()> {
      code: business_code,
      msg: msg.to_string(),
      data: None,
    };
    (status_code, Json(resp)).into_response()
  }
}

pub trait ToAppError<T> {
  fn with_op(self, op: &'static str) -> Result<T, AppError>;
}

impl<T, E: std::error::Error + Send + Sync + 'static> ToAppError<T> for Result<T, E> {
  fn with_op(self, op: &'static str) -> Result<T, AppError> {
    self.map_err(|e| {
      tracing::error!(operation = op, error = ?e, "System Error");
      AppError::Internal {
        msg: format!("Failed to {}", op),
        source: Some(Box::new(e)),
      }
    })
  }
}
