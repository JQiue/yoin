use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::Serialize;

use crate::constants::error_codes::ErrorCode;

pub const SUCCESS_CODE: &str = "ok";

#[derive(Serialize)]
pub struct ApiResponse<T> {
  pub code: String,
  pub msg: String,
  pub data: Option<T>,
}

impl<T> ApiResponse<T> {
  pub fn success(data: T) -> Self {
    Self {
      code: SUCCESS_CODE.to_string(),
      msg: "success".to_string(),
      data: Some(data),
    }
  }

  pub fn error(code: ErrorCode, msg: String) -> Self {
    Self {
      code: code.as_str().to_string(),
      msg,
      data: None,
    }
  }
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
  fn into_response(self) -> axum::response::Response {
    (StatusCode::OK, Json(self)).into_response()
  }
}
