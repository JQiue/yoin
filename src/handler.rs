use axum::{
  Json,
  extract::{FromRequest, Request, State},
};
use serde::{Deserialize, Serialize};

use crate::{
  AppState,
  entity::sites::SiteConfig,
  error::AppError,
  middleware::{OptionnalAuth, RequireAuth},
  response::ApiResponse,
  service::{create_user, get_all_sites, get_token, get_user_profile, update_user_profile},
};

pub struct AppJson<T>(pub T);

impl<S, T> FromRequest<S> for AppJson<T>
where
  axum::Json<T>: FromRequest<S, Rejection = axum::extract::rejection::JsonRejection>,
  S: Send + Sync,
{
  type Rejection = AppError;
  async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
    match axum::Json::<T>::from_request(req, state).await {
      Ok(Json(value)) => Ok(AppJson(value)),
      Err(rejection) => match rejection {
        axum::extract::rejection::JsonRejection::MissingJsonContentType(
          missing_json_content_type,
        ) => Err(AppError::unsupported_media_type(
          missing_json_content_type.body_text(),
        )),
        other => Err(AppError::bad_request(other.body_text())),
      },
    }
  }
}

pub async fn health_check() -> Result<ApiResponse<()>, AppError> {
  Ok(ApiResponse::success(()))
}

#[derive(Deserialize)]
pub struct RegisterRequest {
  email: String,
  password: String,
  nickname: String,
  url: String,
}

#[derive(Serialize)]
pub struct RegisterResponse {
  pub avatar: String,
  pub nickname: String,
  pub url: String,
  pub token: String,
}

pub async fn register(
  State(state): State<AppState>,
  AppJson(payload): AppJson<RegisterRequest>,
) -> Result<ApiResponse<RegisterResponse>, AppError> {
  Ok(ApiResponse::success(
    create_user(
      payload.nickname,
      payload.url,
      payload.email,
      payload.password,
      &state.conn,
      &state.jwt_key,
    )
    .await?,
  ))
}

#[derive(Deserialize)]
pub struct LoginRequest {
  email: String,
  password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
  pub avatar: String,
  pub nickname: String,
  pub url: String,
  pub token: String,
}

pub async fn login(
  State(state): State<AppState>,
  AppJson(payload): AppJson<LoginRequest>,
) -> Result<ApiResponse<LoginResponse>, AppError> {
  Ok(ApiResponse::success(
    get_token(payload.email, payload.password, &state.conn, &state.jwt_key).await?,
  ))
}

#[derive(Serialize)]
pub struct GetMyProfileResponse {
  pub avatar: String,
  pub nickname: String,
  pub url: String,
}

pub async fn get_my_profile(
  State(state): State<AppState>,
  require_auth: RequireAuth,
) -> Result<ApiResponse<GetMyProfileResponse>, AppError> {
  Ok(ApiResponse::success(
    get_user_profile(require_auth.user_id, &state.conn).await?,
  ))
}

#[derive(Deserialize)]
pub struct UpdateMyProfileRequest {
  nickname: Option<String>,
  avatar: Option<String>,
  url: Option<String>,
}

#[derive(Serialize)]
pub struct UpdateMyProfileResponse {
  pub avatar: String,
  pub nickname: String,
  pub url: String,
}

pub async fn update_my_profile(
  State(state): State<AppState>,
  require_auth: RequireAuth,
  AppJson(payload): AppJson<UpdateMyProfileRequest>,
) -> Result<ApiResponse<UpdateMyProfileResponse>, AppError> {
  Ok(ApiResponse::success(
    update_user_profile(
      require_auth.user_id,
      payload.nickname,
      payload.avatar,
      payload.url,
      &state.conn,
    )
    .await?,
  ))
}

#[derive(Serialize)]
pub struct ListSitesResponse {
  pub id: i64,
  pub name: String,
  pub config: SiteConfig,
  pub url: String,
}

pub async fn list_sites(
  State(state): State<AppState>,
  require_auth: RequireAuth,
) -> Result<ApiResponse<Vec<ListSitesResponse>>, AppError> {
  Ok(ApiResponse::success(
    get_all_sites(require_auth.user_id, &state.conn).await?,
  ))
}

pub async fn create_site(
  State(state): State<AppState>,
  require_auth: RequireAuth,
) -> Result<ApiResponse<()>, AppError> {
  Ok(ApiResponse::success(()))
}

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
