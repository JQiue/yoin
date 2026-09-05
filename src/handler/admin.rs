use std::{collections::BTreeMap, sync::Arc};

use axum::extract::{Path, Query, State};
use migration::enums::CommentStatus;
use serde::{Deserialize, Serialize};

use crate::{
  app::AppState,
  entity::comments,
  error::AppError,
  extractor::{AppJson, RequireAuth},
  rbac::permissions::codes::MODERATION_PROVIDER_MANAGE,
  response::ApiResponse,
};

#[derive(Serialize)]
pub struct AdminCapabilitiesView {
  pub global_permissions: Vec<String>,
  pub site_permissions: BTreeMap<String, Vec<String>>,
}

#[derive(Serialize)]
pub struct RoleView {
  pub id: i64,
  pub name: String,
  pub description: Option<String>,
  pub permission_names: Vec<String>,
}

#[derive(Serialize)]
pub struct PermissionView {
  pub id: i64,
  pub name: String,
  pub description: Option<String>,
}

#[derive(Serialize)]
pub struct UserRoleBindingView {
  pub id: i64,
  pub user_id: i64,
  pub role_id: i64,
  pub role_name: Option<String>,
  pub scope_type: String,
  pub scope_id: Option<String>,
  pub created_at: String,
  pub updated_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct CommentViewForAdmin {
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
  pub is_anonymous: bool,
  pub is_private: bool,
  pub avatar: String,
  pub created_at: String,
  pub updated_at: String,
  pub status: CommentStatus,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub replies: Option<Vec<CommentViewForAdmin>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub has_more: Option<bool>,
}

impl CommentViewForAdmin {
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
      is_anonymous: model.is_anonymous,
      is_private: model.is_private,
      status: model.status,
      created_at: model.created_at.and_utc().to_rfc3339(),
      updated_at: model.updated_at.and_utc().to_rfc3339(),
      replies: None,
      has_more: None,
    }
  }
}

pub async fn me_capabilities(
  State(state): State<Arc<AppState>>,
  require_auth: RequireAuth,
) -> Result<ApiResponse<AdminCapabilitiesView>, AppError> {
  Ok(ApiResponse::success(
    state
      .service
      .get_admin_capabilities(require_auth.user_id)
      .await?,
  ))
}

pub async fn list_roles(
  State(state): State<Arc<AppState>>,
  require_auth: RequireAuth,
) -> Result<ApiResponse<Vec<RoleView>>, AppError> {
  Ok(ApiResponse::success(
    state
      .service
      .list_roles_for_admin(require_auth.user_id)
      .await?,
  ))
}

pub async fn list_permissions(
  State(state): State<Arc<AppState>>,
  require_auth: RequireAuth,
) -> Result<ApiResponse<Vec<PermissionView>>, AppError> {
  Ok(ApiResponse::success(
    state
      .service
      .list_permissions_for_admin(require_auth.user_id)
      .await?,
  ))
}

pub async fn list_user_role_bindings(
  State(state): State<Arc<AppState>>,
  require_auth: RequireAuth,
) -> Result<ApiResponse<Vec<UserRoleBindingView>>, AppError> {
  Ok(ApiResponse::success(
    state
      .service
      .list_user_role_bindings_for_admin(require_auth.user_id)
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
  pub status: Option<CommentStatus>,
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
pub async fn list_comments(
  State(state): State<Arc<AppState>>,
  require_auth: RequireAuth,
  Query(qs): Query<ListQueryString>,
) -> Result<ApiResponse<PageResponse<CommentViewForAdmin>>, AppError> {
  state
    .service
    .require_global_permission(require_auth.user_id, MODERATION_PROVIDER_MANAGE)
    .await?;
  Ok(ApiResponse::success(
    state
      .service
      .list_comments_for_admin(require_auth.user_id, qs)
      .await?,
  ))
}

#[derive(Debug, Deserialize)]
pub struct UpdateCommentStatusPayload {
  status: CommentStatus,
}

pub async fn update_comment_status(
  State(state): State<Arc<AppState>>,
  require_auth: RequireAuth,
  Path(id): Path<i64>,
  payload: AppJson<UpdateCommentStatusPayload>,
) -> Result<ApiResponse<()>, AppError> {
  Ok(ApiResponse::success(
    state
      .service
      .update_comment_status_for_admin(require_auth.user_id, id, payload.0.status)
      .await?,
  ))
}

// pub async fn delete(
//   State(state): State<Arc<AppState>>,
//   require_auth: RequireAuth,
//   Path(id): Path<i64>,
// ) -> Result<ApiResponse<()>, AppError> {
//   state
//     .service
//     .delete_comment(require_auth.user_id, id)
//     .await?;
//   Ok(ApiResponse::success(()))
// }
