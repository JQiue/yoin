use std::sync::Arc;

use axum::extract::{Path, Query, State};
use serde::{Deserialize, Serialize};

use crate::{
  app::{AppState, UserKey},
  entity::comments,
  error::AppError,
  extractor::{AppJson, OptionnalAuth, RemoteIp, RequireAuth},
  response::ApiResponse,
};

#[derive(Debug, Deserialize)]
pub struct CreateCommentPayload {
  pub site_id: i64,
  pub nickname: String,
  pub website: String,
  pub content: String,
  pub page_path: String,
  pub email: String,
  pub parent_id: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct CommentView {
  pub id: i64,
  pub thread_id: Option<i64>,
  pub parent_id: Option<i64>,
  pub nickname: String,
  pub website: String,
  pub content: String,
  pub up_vote: i32,
  pub down_vote: i32,
  pub device: String,
  pub location: String,
  pub is_sticky: bool,
  pub avatar: String,
  pub created_at: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub replies: Option<Vec<CommentView>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub has_more: Option<bool>,
}

impl CommentView {
  pub fn from_model(model: comments::Model) -> Self {
    let parser = pulldown_cmark::Parser::new(&model.content);
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);
    Self {
      id: model.id,
      thread_id: model.thread_id,
      parent_id: model.parent_id,
      nickname: model.nickname,
      website: model.website,
      content: html_output,
      avatar: model.avatar,
      up_vote: model.up_vote,
      down_vote: model.down_vote,
      device: model.device,
      location: model.location,
      is_sticky: model.is_sticky,
      created_at: model.created_at.and_utc().to_rfc3339(),
      replies: None,
      has_more: None,
    }
  }
}

pub async fn create(
  State(state): State<Arc<AppState>>,
  optional_auth: OptionnalAuth,
  remote_ip: RemoteIp,
  AppJson(payload): AppJson<CreateCommentPayload>,
) -> Result<ApiResponse<CommentView>, AppError> {
  let site_config = state.get_site_config(payload.site_id).await?;

  if !state.is_admin(optional_auth.user_id.unwrap_or(-1)).await {
    if optional_auth.user_id.is_none() && !site_config.allow_anonymous {
      return Err(AppError::forbidden(
        "anonymous access not allowed".to_string(),
      ));
    }

    let key = match optional_auth.user_id {
      Some(id) => (payload.site_id, UserKey::UserId(id)),
      None => (payload.site_id, UserKey::Ip(remote_ip.ip)),
    };

    if state
      .comment_rate_limiter
      .check_rate_limit(key, site_config.comment_limit_seconds)
      .await
    {
      return Err(AppError::bad_request(
        "The comment is too fast.".to_string(),
      ));
    };

    if payload.content.chars().count() > site_config.max_comment_length {
      return Err(AppError::bad_request("comment too long".to_string()));
    }
  }

  Ok(ApiResponse::success(
    state
      .service
      .create_comment(optional_auth.user_id, payload)
      .await?,
  ))
}

#[derive(Debug, Deserialize)]
pub struct ListQueryString {
  pub site_id: i64,
  pub page_path: String,
  pub page_size: u64,
  pub page_offset: u64,
  pub sort: String,
}

#[derive(Serialize, Deserialize)]
pub struct PageResponse<T> {
  pub items: Vec<T>,
  pub page_size: u64,
  pub page_offset: u64,
  pub total: u64,
  pub total_pages: u64,
}

// FIXME: Cursor pagination
pub async fn list(
  State(state): State<Arc<AppState>>,
  Query(qs): Query<ListQueryString>,
) -> Result<ApiResponse<PageResponse<CommentView>>, AppError> {
  Ok(ApiResponse::success(state.service.list_comments(qs).await?))
}

// FIXME: Cursor pagination
pub async fn list_replies(
  State(state): State<Arc<AppState>>,
  Path(id): Path<i64>,
  Query(qs): Query<ListQueryString>,
) -> Result<ApiResponse<PageResponse<CommentView>>, AppError> {
  Ok(ApiResponse::success(
    state.service.list_replies(id, qs).await?,
  ))
}

pub async fn delete(
  State(state): State<Arc<AppState>>,
  require_auth: RequireAuth,
  Path(id): Path<i64>,
) -> Result<ApiResponse<()>, AppError> {
  state
    .service
    .delete_comment(require_auth.user_id, id)
    .await?;
  Ok(ApiResponse::success(()))
}

pub async fn vote(
  State(state): State<Arc<AppState>>,
  Path((id, r#type)): Path<(i64, String)>,
) -> Result<ApiResponse<()>, AppError> {
  Ok(ApiResponse::success(
    state.service.update_vote(id, r#type).await?,
  ))
}
