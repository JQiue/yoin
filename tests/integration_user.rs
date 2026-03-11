mod common;

use axum::http::StatusCode;
use serde::Deserialize;
use serde_json::json;

use common::{
  ApiResponse, get_with_bearer, patch_json_with_bearer, read_json, register_user, test_app,
};

#[derive(Deserialize)]
struct UserProfile {
  avatar: String,
  nickname: String,
  website: String,
  email: String,
}

#[tokio::test]
async fn fetch_profile_returns_current_user() {
  let app = test_app().await;
  let user = register_user(&app, "profile@example.com", "secret123")
    .await
    .data
    .expect("user");

  let resp = get_with_bearer(&app, "/api/users/me", &user.token).await;
  assert_eq!(resp.status(), StatusCode::OK);

  let body: ApiResponse<UserProfile> = read_json(resp).await;
  let profile = body.data.expect("profile");
  assert_eq!(profile.nickname, "tester");
  assert_eq!(profile.email, "profile@example.com");
  assert_eq!(profile.website, "");
  assert!(!profile.avatar.is_empty());
}

#[tokio::test]
async fn update_profile_persists_latest_fields() {
  let app = test_app().await;
  let user = register_user(&app, "profile-update@example.com", "secret123")
    .await
    .data
    .expect("user");

  let resp = patch_json_with_bearer(
    &app,
    "/api/users/me",
    &user.token,
    json!({
      "nickname": "updated tester",
      "website": "https://example.com",
      "avatar": "https://cdn.example.com/avatar.png"
    }),
  )
  .await;
  assert_eq!(resp.status(), StatusCode::OK);

  let body: ApiResponse<UserProfile> = read_json(resp).await;
  let profile = body.data.expect("profile");
  assert_eq!(profile.nickname, "updated tester");
  assert_eq!(profile.website, "https://example.com");
  assert_eq!(profile.avatar, "https://cdn.example.com/avatar.png");
  assert_eq!(profile.email, "profile-update@example.com");
}
