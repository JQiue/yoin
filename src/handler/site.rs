use std::sync::Arc;

use axum::extract::State;
use serde::{Deserialize, Serialize};

use crate::{
  AppState,
  entity::sites::SiteConfig,
  error::AppError,
  extractor::{AppJson, RequireAuth},
  response::ApiResponse,
  service::{create_site, list_sites, update_site},
};

#[derive(Serialize)]
pub struct SiteView {
  pub id: i64,
  pub name: String,
  pub config: SiteConfig,
  pub url: String,
}

pub async fn list(
  State(state): State<Arc<AppState>>,
  require_auth: RequireAuth,
) -> Result<ApiResponse<Vec<SiteView>>, AppError> {
  Ok(ApiResponse::success(
    list_sites(require_auth.user_id, &state.conn).await?,
  ))
}

#[derive(Deserialize)]
pub struct CreateSitePayload {
  pub name: String,
  pub url: String,
}

pub async fn create(
  State(state): State<Arc<AppState>>,
  require_auth: RequireAuth,
  AppJson(payload): AppJson<CreateSitePayload>,
) -> Result<ApiResponse<SiteView>, AppError> {
  Ok(ApiResponse::success(
    create_site(require_auth.user_id, payload, &state.conn).await?,
  ))
}

#[derive(Deserialize)]
pub struct UpdateSitePayload {
  pub id: i64,
  pub name: Option<String>,
  pub url: Option<String>,
  pub config: Option<SiteConfig>,
}

pub async fn update(
  State(state): State<Arc<AppState>>,
  require_auth: RequireAuth,
  AppJson(payload): AppJson<UpdateSitePayload>,
) -> Result<ApiResponse<SiteView>, AppError> {
  let resp = ApiResponse::success(update_site(require_auth.user_id, payload, &state.conn).await?);
  state.preload_configs().await;
  Ok(resp)
}
