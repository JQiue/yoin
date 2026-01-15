use axum::extract::State;
use serde::{Deserialize, Serialize};

use crate::{
  AppState,
  error::AppError,
  extractor::{AppJson, OptionnalAuth},
  response::ApiResponse,
};

#[derive(Debug, Deserialize)]
pub struct CreateCommentRequest {
  site_id: i64,
  nickname: String,
  link: String,
  content: String,
}

#[derive(Serialize)]
pub struct CreateCommentResponse {}

pub async fn create_comment(
  State(state): State<AppState>,
  optional_auth: OptionnalAuth,
  AppJson(payload): AppJson<CreateCommentRequest>,
) -> Result<ApiResponse<CreateCommentResponse>, AppError> {
  println!("{:?}", optional_auth);
  println!("{:?}", payload);
  println!(
    "{:?}",
    state.site_config.lock().unwrap().get(&payload.site_id)
  );
  Ok(ApiResponse::success(CreateCommentResponse {}))
}
