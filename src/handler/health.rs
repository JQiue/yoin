use crate::{error::AppError, response::ApiResponse};

pub async fn health_check() -> Result<ApiResponse<()>, AppError> {
  Ok(ApiResponse::success(()))
}
