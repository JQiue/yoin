use std::{
  net::{IpAddr, SocketAddr},
  sync::Arc,
};

use axum::{
  Json,
  extract::{ConnectInfo, FromRequest, Request},
  http::{HeaderMap, HeaderValue, header::COOKIE},
  middleware::Next,
  response::Response,
};
use helpers::uuid::{Alphabet, nanoid};

use crate::{
  app::AppState,
  constants::guest::{COOKIE_NAME, HEADER_NAME, ID_MAX_LEN, ID_MIN_LEN},
  error::AppError,
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

use axum::{
  extract::FromRequestParts,
  http::{StatusCode, header::AUTHORIZATION, request::Parts},
};
use helpers::jwt;

pub struct RequireAuth {
  pub user_id: i64,
}

fn parse_bearer_user_id(parts: &Parts, jwt_key: &str) -> Result<Option<i64>, StatusCode> {
  let auth_header = parts
    .headers
    .get(AUTHORIZATION)
    .and_then(|value| value.to_str().ok());

  let Some(auth_header) = auth_header else {
    return Ok(None);
  };

  if !auth_header.starts_with("Bearer ") {
    return Ok(None);
  }

  let token = auth_header.trim_start_matches("Bearer ").trim();
  if token.is_empty() {
    return Ok(None);
  }
  let data = jwt::verify(token, jwt_key).map_err(|_| StatusCode::UNAUTHORIZED)?;
  Ok(Some(data.claims.data))
}

impl FromRequestParts<Arc<AppState>> for RequireAuth {
  type Rejection = StatusCode;

  async fn from_request_parts(
    parts: &mut Parts,
    state: &Arc<AppState>,
  ) -> Result<Self, Self::Rejection> {
    let user_id = parse_bearer_user_id(parts, &state.jwt_key)?.ok_or(StatusCode::UNAUTHORIZED)?;
    Ok(RequireAuth { user_id })
  }
}

#[derive(Debug)]
pub struct OptionnalAuth {
  pub user_id: Option<i64>,
}

impl FromRequestParts<Arc<AppState>> for OptionnalAuth {
  type Rejection = StatusCode;

  async fn from_request_parts(
    parts: &mut Parts,
    state: &Arc<AppState>,
  ) -> Result<Self, Self::Rejection> {
    let user_id = parse_bearer_user_id(parts, &state.jwt_key)?;
    Ok(OptionnalAuth { user_id })
  }
}

#[derive(Debug)]
pub struct RemoteIp {
  pub ip: String,
}

fn is_trusted_proxy(ip: &IpAddr) -> bool {
  match ip {
    IpAddr::V4(v4) => v4.is_loopback() || v4.is_private() || v4.is_link_local(),
    IpAddr::V6(v6) => v6.is_loopback() || v6.is_unique_local(),
  }
}

impl<S> FromRequestParts<S> for RemoteIp
where
  S: Send + Sync,
{
  type Rejection = StatusCode;

  async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
    let peer_ip = parts
      .extensions
      .get::<ConnectInfo<SocketAddr>>()
      .map(|ConnectInfo(addr)| addr.ip())
      .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

    if is_trusted_proxy(&peer_ip)
      && let Some(forwarded_ip) = parts
        .headers
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.split(',').next())
        .and_then(|s| s.trim().parse::<IpAddr>().ok())
    {
      return Ok(RemoteIp {
        ip: forwarded_ip.to_string(),
      });
    }

    Ok(RemoteIp {
      ip: peer_ip.to_string(),
    })
  }
}

#[derive(Clone, Debug)]
pub struct GuestId {
  pub value: String,
  pub is_new: bool,
}

pub fn is_valid_guest_id(id: &str) -> bool {
  let len = id.len();
  (ID_MIN_LEN..=ID_MAX_LEN).contains(&len)
    && id
      .chars()
      .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
}

fn cookie_value(headers: &HeaderMap, name: &str) -> Option<String> {
  let cookie = headers.get(COOKIE)?.to_str().ok()?;
  cookie.split(';').find_map(|part| {
    let part = part.trim();
    let (key, value) = part.split_once('=')?;
    (key == name).then(|| value.to_string())
  })
}

pub async fn ensure_guest_id(mut req: Request, next: Next) -> Response {
  let header_id = req
    .headers()
    .get(HEADER_NAME)
    .and_then(|value| value.to_str().ok())
    .map(str::to_string);
  let cookie_id = cookie_value(req.headers(), COOKIE_NAME);
  let (value, is_new) = match header_id.or(cookie_id) {
    Some(id) if is_valid_guest_id(&id) => (id, false),
    _ => (nanoid(&Alphabet::DEFAULT, 21), true),
  };

  req.extensions_mut().insert(GuestId {
    value: value.clone(),
    is_new,
  });
  let mut res = next.run(req).await;

  if let Ok(header_value) = HeaderValue::from_str(&value) {
    res.headers_mut().insert(HEADER_NAME, header_value);
  }
  if is_new {
    let cookie = format!("{COOKIE_NAME}={value}; Path=/; Max-Age=31536000; HttpOnly; SameSite=Lax");
    if let Ok(header_value) = HeaderValue::from_str(&cookie) {
      res
        .headers_mut()
        .append(axum::http::header::SET_COOKIE, header_value);
    }
  }
  res
}

impl<S> FromRequestParts<S> for GuestId
where
  S: Send + Sync,
{
  type Rejection = StatusCode;

  async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
    parts
      .extensions
      .get::<GuestId>()
      .cloned()
      .ok_or(StatusCode::INTERNAL_SERVER_ERROR)
  }
}
