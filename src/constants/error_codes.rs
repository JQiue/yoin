use axum::http::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ErrorCode {
  BadRequest,
  Unauthorized,
  Forbidden,
  NotFound,
  Conflict,
  UnsupportedMediaType,
  RateLimited,
  InternalError,
  InvalidCredentials,
  UserNotFound,
  UserAlreadyExists,
  SiteNotFound,
  CommentNotFound,
  ModerationProviderNotFound,
  InvalidOauthProvider,
}

impl ErrorCode {
  pub fn as_str(self) -> &'static str {
    match self {
      Self::BadRequest => "bad_request",
      Self::Unauthorized => "unauthorized",
      Self::Forbidden => "forbidden",
      Self::NotFound => "not_found",
      Self::Conflict => "conflict",
      Self::UnsupportedMediaType => "unsupported_media_type",
      Self::RateLimited => "rate_limited",
      Self::InternalError => "internal_error",
      Self::InvalidCredentials => "invalid_credentials",
      Self::UserNotFound => "user_not_found",
      Self::UserAlreadyExists => "user_already_exists",
      Self::SiteNotFound => "site_not_found",
      Self::CommentNotFound => "comment_not_found",
      Self::ModerationProviderNotFound => "moderation_provider_not_found",
      Self::InvalidOauthProvider => "invalid_oauth_provider",
    }
  }

  pub fn http_status(&self) -> StatusCode {
    match self {
      Self::BadRequest => StatusCode::BAD_REQUEST,
      Self::Unauthorized => StatusCode::UNAUTHORIZED,
      Self::Forbidden => StatusCode::FORBIDDEN,
      Self::NotFound => StatusCode::NOT_FOUND,
      Self::Conflict => StatusCode::CONFLICT,
      Self::UnsupportedMediaType => StatusCode::UNSUPPORTED_MEDIA_TYPE,
      Self::RateLimited => StatusCode::TOO_MANY_REQUESTS,
      Self::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
      Self::InvalidCredentials => StatusCode::UNAUTHORIZED,
      Self::UserNotFound => StatusCode::NOT_FOUND,
      Self::UserAlreadyExists => StatusCode::CONFLICT,
      Self::SiteNotFound => StatusCode::NOT_FOUND,
      Self::CommentNotFound => StatusCode::NOT_FOUND,
      Self::ModerationProviderNotFound => StatusCode::NOT_FOUND,
      Self::InvalidOauthProvider => StatusCode::BAD_REQUEST,
    }
  }
}
