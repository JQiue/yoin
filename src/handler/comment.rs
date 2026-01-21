use std::sync::Arc;

use axum::extract::{Query, State};
use serde::{Deserialize, Serialize};

use crate::{
  AppState,
  error::AppError,
  extractor::{AppJson, OptionnalAuth, RemoteIp},
  response::ApiResponse,
  service::{create_comment, list_comments},
};

#[derive(Debug, Deserialize)]
pub struct CreateCommentPayload {
  pub site_id: i64,
  pub nickname: String,
  pub link: String,
  pub content: String,
  pub page_page: String,
  pub email: String,
  pub rid: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct CommentView {
  pub id: i64,
  pub rid: i64,
  pub nickname: String,
  pub link: String,
  pub content: String,
  pub up_vote: i32,
  pub down_vote: i32,
  pub device: String,
  pub location: String,
  pub is_sticky: bool,
  pub created_at: String,
}

pub async fn create(
  State(state): State<Arc<AppState>>,
  optional_auth: OptionnalAuth,
  remote_ip: RemoteIp,
  AppJson(payload): AppJson<CreateCommentPayload>,
) -> Result<ApiResponse<CommentView>, AppError> {
  let site_config = state
    .site_config
    .lock()
    .await
    .get(&payload.site_id)
    .ok_or(AppError::Internal {
      msg: "site config not found".to_string(),
      source: None,
    })?
    .clone();

  let is_admin = state.is_admin(optional_auth.user_id.unwrap_or(-1)).await;

  if !is_admin {
    if optional_auth.user_id.is_none() && !site_config.allow_anonymous {
      return Err(AppError::forbidden(
        "anonymous access not allowed".to_string(),
      ));
    }
    state
      .check_rate_limit(payload.site_id, optional_auth.user_id, remote_ip.ip, 10)
      .await?;
    if payload.content.chars().count() > site_config.max_comment_length {
      return Err(AppError::bad_request("comment too long".to_string()));
    }
  }

  Ok(ApiResponse::success(
    create_comment(optional_auth.user_id, payload, &state.conn).await?,
  ))
}

#[derive(Debug, Deserialize)]
pub struct ListQueryString {
  pub site_id: i64,
  pub page_path: String,
  pub sort_by: String,
  pub limit: u64,
  pub offset: u64,
}

#[derive(Serialize, Deserialize)]
pub struct PageResponse<T> {
  pub items: Vec<T>,
  pub page_size: u64,
  pub page: u64,
  pub total: u64,
}

#[axum::debug_handler]
pub async fn list(
  State(state): State<Arc<AppState>>,
  Query(qs): Query<ListQueryString>,
) -> Result<ApiResponse<PageResponse<CommentView>>, AppError> {
  Ok(ApiResponse::success(list_comments(qs, &state.conn).await?))
}
