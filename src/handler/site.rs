use axum::extract::State;
use serde::{Deserialize, Serialize};

use crate::{
  AppState,
  entity::sites::SiteConfig,
  error::AppError,
  extractor::{AppJson, RequireAuth},
  response::ApiResponse,
  service::{create_site, list_sites},
};

#[derive(Serialize)]
pub struct SiteView {
  pub id: i64,
  pub name: String,
  pub config: SiteConfig,
  pub url: String,
}

pub async fn list(
  State(state): State<AppState>,
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
  State(state): State<AppState>,
  require_auth: RequireAuth,
  AppJson(payload): AppJson<CreateSitePayload>,
) -> Result<ApiResponse<SiteView>, AppError> {
  Ok(ApiResponse::success(
    create_site(require_auth.user_id, payload, &state.conn).await?,
  ))
}
