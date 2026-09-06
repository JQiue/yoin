use axum::{
  Json,
  http::StatusCode,
  response::{IntoResponse, Response},
};
use thiserror::Error;

use crate::{constants::error_codes::ErrorCode, response::ApiResponse};

#[derive(Debug, Error)]
pub enum AppError {
  #[error("{msg}")]
  Client { code: ErrorCode, msg: String },
  #[error("Internal error: {msg}")]
  Internal {
    msg: String,
    source: Option<Box<dyn std::error::Error + Send + Sync>>,
  },
}

impl AppError {
  pub fn bad_request(msg: String) -> Self {
    Self::Client {
      code: ErrorCode::BadRequest,
      msg,
    }
  }

  pub fn forbidden(msg: String) -> Self {
    Self::Client {
      code: ErrorCode::Forbidden,
      msg,
    }
  }

  pub fn invalid_credentials(msg: String) -> Self {
    Self::Client {
      code: ErrorCode::InvalidCredentials,
      msg,
    }
  }

  pub fn unsupported_media_type(msg: String) -> Self {
    Self::Client {
      code: ErrorCode::UnsupportedMediaType,
      msg,
    }
  }

  pub fn user_not_found(msg: String) -> Self {
    Self::Client {
      code: ErrorCode::UserNotFound,
      msg,
    }
  }

  pub fn user_already_exists(msg: String) -> Self {
    Self::Client {
      code: ErrorCode::UserAlreadyExists,
      msg,
    }
  }

  pub fn site_not_found(msg: String) -> Self {
    Self::Client {
      code: ErrorCode::SiteNotFound,
      msg,
    }
  }

  pub fn comment_not_found(msg: String) -> Self {
    Self::Client {
      code: ErrorCode::CommentNotFound,
      msg,
    }
  }

  pub fn moderation_provider_not_found(msg: String) -> Self {
    Self::Client {
      code: ErrorCode::ModerationProviderNotFound,
      msg,
    }
  }

  pub fn invalid_oauth_provider(msg: String) -> Self {
    Self::Client {
      code: ErrorCode::InvalidOauthProvider,
      msg,
    }
  }
}

impl IntoResponse for AppError {
  fn into_response(self) -> Response {
    let (status, code, msg) = match self {
      AppError::Client { code, msg } => (code.http_status(), code, msg),
      AppError::Internal { .. } => {
        tracing::error!("Detailed Error: {:?}", self);
        (
          StatusCode::INTERNAL_SERVER_ERROR,
          ErrorCode::InternalError,
          "An internal server error occurred".to_string(),
        )
      }
    };
    (status, Json(ApiResponse::<()>::error(code, msg))).into_response()
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
