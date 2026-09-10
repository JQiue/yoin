use axum::{body::Body, http::StatusCode};
use serde_json::Value;

use crate::common::{
  self, ApiResponse, build_request, get_with_bearer, login_user, read_json, register_user, request,
  test_app,
};

#[tokio::test]
async fn register_first_user_can_access_sites_as_admin() {
  let app = test_app().await;
  let resp = register_user(&app, "admin@example.com", "secret123").await;

  assert_eq!(resp.code, "ok");
  assert_eq!(resp.msg, "success");
  let data = resp.data.expect("register data");
  let resp = get_with_bearer(&app, "/api/sites", &data.token).await;
  assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test]
async fn login_returns_token_for_existing_user() {
  let app = test_app().await;
  let _ = register_user(&app, "user@example.com", "secret123").await;
  let body = login_user(&app, "user@example.com", "secret123").await;
  assert_eq!(body.code, "ok");
  assert!(body.data.expect("login data").token.len() > 10);
}

#[tokio::test]
async fn sites_requires_authorization() {
  let app = test_app().await;
  let req = build_request("GET", "/api/sites", Body::empty());
  let resp = request(&app, req).await;
  assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn login_with_wrong_password_returns_unauthorized() {
  let app = test_app().await;
  let _ = register_user(&app, "user2@example.com", "secret123").await;
  let body = login_user(&app, "user2@example.com", "wrong-password").await;
  assert_eq!(body.code, "invalid_credentials");
  assert!(body.data.is_none());
}

#[tokio::test]
async fn login_error_response_uses_unauthorized_status() {
  let app = test_app().await;
  let _ = register_user(&app, "user3@example.com", "secret123").await;
  let resp = common::post_json(
    &app,
    "/api/auth/login",
    serde_json::json!({
      "email": "user3@example.com",
      "password": "wrong-password"
    }),
  )
  .await;
  assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
  let body: ApiResponse<Value> = read_json(resp).await;
  assert_eq!(body.code, "invalid_credentials");
}
