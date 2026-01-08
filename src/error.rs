use axum::{
  Json,
  http::StatusCode,
  response::{IntoResponse, Response},
};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
  #[error("{msg}")]
  Client {
    kind: ClientErrorKind,
    msg: &'static str,
  },
  #[error("Internal error: {msg}")]
  Internal {
    msg: String,
    source: Option<Box<dyn std::error::Error + Send + Sync>>,
  },
}

#[derive(Debug)]
pub enum ClientErrorKind {
  BadRequest,   // 400
  Unauthorized, // 401
  Forbidden,    // 403
  NotFound,     // 404
  Conflict,     // 409
  RateLimited,  // 429
}

impl AppError {
  pub fn not_found(msg: &'static str) -> Self {
    Self::Client {
      kind: ClientErrorKind::NotFound,
      msg,
    }
  }

  pub fn conflict(msg: &'static str) -> Self {
    Self::Client {
      kind: ClientErrorKind::Conflict,
      msg,
    }
  }
}

impl IntoResponse for AppError {
  fn into_response(self) -> Response {
    let (status_code, msg) = match self {
      AppError::Client { kind, msg } => {
        let status_code = match kind {
          ClientErrorKind::BadRequest => StatusCode::BAD_REQUEST,
          ClientErrorKind::Unauthorized => StatusCode::UNAUTHORIZED,
          ClientErrorKind::Forbidden => StatusCode::FORBIDDEN,
          ClientErrorKind::NotFound => StatusCode::NOT_FOUND,
          ClientErrorKind::Conflict => StatusCode::CONFLICT,
          ClientErrorKind::RateLimited => StatusCode::TOO_MANY_REQUESTS,
        };
        (status_code, msg)
      }
      AppError::Internal { .. } => (
        StatusCode::INTERNAL_SERVER_ERROR,
        "An internal server error occurred",
      ),
    };
    (status_code, Json(json!({ "error": { "msg": msg } }))).into_response()
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
