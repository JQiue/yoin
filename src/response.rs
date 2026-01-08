use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T> {
  pub code: i32,       // 0=成功，非0=业务错误码
  pub msg: String,     // 描述信息
  pub data: Option<T>, // 成功时有数据
}

impl<T> ApiResponse<T> {
  pub fn success(data: T) -> Self {
    Self {
      code: 0,
      msg: "success".to_string(),
      data: Some(data),
    }
  }
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
  fn into_response(self) -> axum::response::Response {
    (StatusCode::OK, Json(self)).into_response()
  }
}
