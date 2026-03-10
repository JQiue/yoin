use std::sync::Arc;

use axum::extract::{Path, Query, State};
use helpers::jwt;
use serde::{Deserialize, Serialize};

use crate::{
  app::AppState,
  error::AppError,
  extractor::{AppJson, RequireAuth},
  helper::OauthService,
  rbac::permissions::codes::OAUTH_PROVIDER_MANAGE,
  response::ApiResponse,
};

#[derive(Deserialize)]
pub struct RegisterPayload {
  email: String,
  password: String,
  nickname: String,
  website: String,
}

#[derive(Serialize)]
pub struct UserWithToken {
  pub token: String,
  #[serde(flatten)]
  pub user: UserProfile,
}

#[derive(Serialize)]
pub struct UserProfile {
  pub avatar: String,
  pub nickname: String,
  pub website: String,
  pub email: String,
}

pub async fn register(
  State(state): State<Arc<AppState>>,
  AppJson(payload): AppJson<RegisterPayload>,
) -> Result<ApiResponse<UserWithToken>, AppError> {
  Ok(ApiResponse::success(
    state
      .service
      .create_user(
        payload.nickname,
        payload.website,
        payload.email,
        payload.password,
        &state.jwt_key,
      )
      .await?,
  ))
}

#[derive(Deserialize)]
pub struct LoginPayload {
  pub email: String,
  pub password: String,
}

pub async fn login(
  State(state): State<Arc<AppState>>,
  AppJson(payload): AppJson<LoginPayload>,
) -> Result<ApiResponse<UserWithToken>, AppError> {
  Ok(ApiResponse::success(
    state.service.login_user(payload, &state.jwt_key).await?,
  ))
}

#[derive(Debug, Deserialize)]
pub struct CreateOauthProviderPayload {
  pub provider_code: String,
  pub client_id: String,
  pub client_secret: String,
  pub redirect_uri: String,
}

pub async fn create_oauth_provider(
  State(state): State<Arc<AppState>>,
  require_auth: RequireAuth,
  AppJson(payload): AppJson<CreateOauthProviderPayload>,
) -> Result<ApiResponse<()>, AppError> {
  state
    .service
    .require_global_permission(require_auth.user_id, OAUTH_PROVIDER_MANAGE)
    .await?;
  state.service.create_oauth_provider(payload).await?;
  Ok(ApiResponse::success(()))
}

#[derive(Serialize)]
pub struct OauthStartPayload {
  pub auth_url: String,
}

struct OauthProviderConfig<'a> {
  client_id: &'a str,
  client_secret: &'a str,
  redirect_uri: &'a str,
}

fn oauth_provider_config(provider: &str) -> Result<OauthProviderConfig<'static>, AppError> {
  match provider {
    "github" => Ok(OauthProviderConfig {
      client_id: "Ov23liNlcMlxtbP4v5re",
      client_secret: "69d9bd3f716e74143df12c519772fee64102f803",
      redirect_uri: "http://localhost:7410/api/auth/oauth/github/callback",
    }),
    "qq" => Err(AppError::bad_request(
      "qq provider config not set".to_string(),
    )),
    _ => Err(AppError::bad_request("invalid oauth provider".to_string())),
  }
}

#[derive(Debug, Serialize, Deserialize)]
struct OauthJWTClaims {
  provider: String,
}

pub async fn oauth_start(
  State(state): State<Arc<AppState>>,
  Path(provider): Path<String>,
) -> Result<ApiResponse<OauthStartPayload>, AppError> {
  let provider_config = oauth_provider_config(&provider)?;
  let payload = OauthJWTClaims {
    provider: provider.clone(),
  };
  let state = jwt::sign(payload, &state.jwt_key, 180).unwrap();
  let mut oauth = OauthService::new(&provider)?;
  oauth.config.state = &state;
  oauth.config.client_id = provider_config.client_id;
  oauth.config.redirect_uri = provider_config.redirect_uri;
  let auth_url = oauth.authorize_url()?;
  Ok(ApiResponse::success(OauthStartPayload { auth_url }))
}

#[derive(Debug, Deserialize)]
pub struct OauthCallbackQueryString {
  pub code: String,
  pub state: String,
}

pub async fn oauth_callback(
  State(state): State<Arc<AppState>>,
  Path(provider): Path<String>,
  Query(qs): Query<OauthCallbackQueryString>,
) -> Result<ApiResponse<()>, AppError> {
  let provider_config = oauth_provider_config(&provider)?;
  let token_data = jwt::verify::<OauthJWTClaims>(&qs.state, &state.jwt_key)
    .map_err(|_| AppError::forbidden("token invalid or expired".to_string()))?;

  if token_data.claims.data.provider != provider {
    return Err(AppError::forbidden("oauth provider mismatch".to_string()));
  }

  let mut oauth = OauthService::new(&provider)?;
  oauth.config.client_id = provider_config.client_id;
  oauth.config.client_secret = provider_config.client_secret;
  oauth.config.redirect_uri = provider_config.redirect_uri;
  oauth.config.code = &qs.code;
  let access_token = oauth.exchange_code();
  let _user_profile = oauth.fetch_profile(access_token)?;
  Ok(ApiResponse::success(()))
}
