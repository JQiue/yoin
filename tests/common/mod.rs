use std::net::SocketAddr;

use axum::{
  body::Body,
  extract::ConnectInfo,
  http::{Request, header},
  response::Response,
};
use http_body_util::BodyExt;
use serde::Deserialize;
use serde_json::{Value, json};
use tower::ServiceExt;
use yoin::{app::app, db::migrate};

pub const TEST_JWT_KEY: &str = "test_jwt_key_123456789012345678901234567890";

#[derive(Deserialize)]
pub struct ApiResponse<T> {
  pub code: String,
  pub msg: String,
  pub data: Option<T>,
}

#[derive(Deserialize)]
pub struct UserWithToken {
  pub token: String,
}

#[derive(Deserialize)]
pub struct SiteView {
  pub id: i64,
  pub name: String,
  pub url: String,
  pub config: yoin::entity::sites::SiteConfig,
}

pub async fn test_app() -> axum::Router {
  let conn = Box::leak(Box::new(migrate("sqlite::memory:").await.unwrap()));
  app(conn, TEST_JWT_KEY.to_string()).await.unwrap()
}

pub fn build_request(method: &str, uri: &str, body: Body) -> Request<Body> {
  let mut req = Request::builder()
    .method(method)
    .uri(uri)
    .body(body)
    .unwrap();
  req
    .extensions_mut()
    .insert(ConnectInfo(SocketAddr::from(([127, 0, 0, 1], 3000))));
  req
}

pub async fn read_json<T: for<'de> Deserialize<'de>>(resp: Response) -> T {
  let bytes = resp.into_body().collect().await.unwrap().to_bytes();
  serde_json::from_slice(&bytes).unwrap()
}

pub async fn request(app: &axum::Router, req: Request<Body>) -> Response {
  app.clone().oneshot(req).await.unwrap()
}

pub async fn post_json(app: &axum::Router, uri: &str, payload: Value) -> Response {
  let mut req = build_request("POST", uri, Body::from(payload.to_string()));
  req
    .headers_mut()
    .insert(header::CONTENT_TYPE, "application/json".parse().unwrap());
  request(app, req).await
}

pub async fn post_json_with_bearer(
  app: &axum::Router,
  uri: &str,
  token: &str,
  payload: Value,
) -> Response {
  let mut req = build_request("POST", uri, Body::from(payload.to_string()));
  req.headers_mut().insert(
    header::AUTHORIZATION,
    format!("Bearer {}", token).parse().unwrap(),
  );
  req
    .headers_mut()
    .insert(header::CONTENT_TYPE, "application/json".parse().unwrap());
  request(app, req).await
}

pub async fn patch_json_with_bearer(
  app: &axum::Router,
  uri: &str,
  token: &str,
  payload: Value,
) -> Response {
  let mut req = build_request("PATCH", uri, Body::from(payload.to_string()));
  req.headers_mut().insert(
    header::AUTHORIZATION,
    format!("Bearer {}", token).parse().unwrap(),
  );
  req
    .headers_mut()
    .insert(header::CONTENT_TYPE, "application/json".parse().unwrap());
  request(app, req).await
}

pub async fn get_with_bearer(app: &axum::Router, uri: &str, token: &str) -> Response {
  let mut req = build_request("GET", uri, Body::empty());
  req.headers_mut().insert(
    header::AUTHORIZATION,
    format!("Bearer {}", token).parse().unwrap(),
  );
  request(app, req).await
}

pub async fn register_user(
  app: &axum::Router,
  email: &str,
  password: &str,
) -> ApiResponse<UserWithToken> {
  let payload = json!({
    "email": email,
    "password": password,
    "nickname": "tester",
    "website": ""
  });
  let resp = post_json(app, "/api/auth/register", payload).await;
  read_json(resp).await
}

pub async fn login_user(
  app: &axum::Router,
  email: &str,
  password: &str,
) -> ApiResponse<UserWithToken> {
  let payload = json!({
    "email": email,
    "password": password
  });
  let resp = post_json(app, "/api/auth/login", payload).await;
  read_json(resp).await
}

pub async fn create_site(
  app: &axum::Router,
  token: &str,
  name: &str,
  allow_anonymous: bool,
  comment_limit_seconds: i64,
) -> ApiResponse<SiteView> {
  let payload = json!({
    "name": name,
    "url": format!("https://{}.example.com", name.to_ascii_lowercase()),
    "config": {
      "allow_anonymous": allow_anonymous,
      "max_comment_length": 1024,
      "comment_limit_seconds": comment_limit_seconds
    }
  });
  let resp = post_json_with_bearer(app, "/api/sites", token, payload).await;
  read_json(resp).await
}
