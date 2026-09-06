use std::sync::Arc;

use axum::extract::{Path, State};
use serde::{Deserialize, Serialize};

use crate::{
  app::AppState,
  entity::sites::SiteConfig,
  error::AppError,
  extractor::{AppJson, RequireAuth},
  response::ApiResponse,
};

#[derive(Serialize)]
pub struct SiteView {
  pub id: i64,
  pub name: String,
  pub config: SiteConfig,
  pub url: String,
}

#[derive(Serialize)]
pub struct PublicSiteConfigView {
  pub allow_anonymous: bool,
  pub allow_private: bool,
  pub max_comment_length: usize,
  pub comment_limit_seconds: i64,
  pub allowed_reactions: Vec<String>,
}

pub async fn list(
  State(state): State<Arc<AppState>>,
  require_auth: RequireAuth,
) -> Result<ApiResponse<Vec<SiteView>>, AppError> {
  Ok(ApiResponse::success(
    state.service.list_sites(require_auth.user_id).await?,
  ))
}

pub async fn public_config(
  State(state): State<Arc<AppState>>,
  Path(id): Path<i64>,
) -> Result<ApiResponse<PublicSiteConfigView>, AppError> {
  Ok(ApiResponse::success(
    state.service.get_public_site_config(id).await?,
  ))
}

#[derive(Deserialize)]
pub struct CreateSitePayload {
  pub name: String,
  pub url: String,
  pub config: SiteConfig,
}

pub async fn create(
  State(state): State<Arc<AppState>>,
  require_auth: RequireAuth,
  AppJson(payload): AppJson<CreateSitePayload>,
) -> Result<ApiResponse<SiteView>, AppError> {
  let resp = ApiResponse::success(
    state
      .service
      .create_site(require_auth.user_id, payload)
      .await?,
  );
  state.preload_configs().await?;
  Ok(resp)
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
  let resp = ApiResponse::success(
    state
      .service
      .update_site(require_auth.user_id, payload)
      .await?,
  );
  state.preload_configs().await?;
  Ok(resp)
}
