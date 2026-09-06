use axum::http::StatusCode;
use serde::Deserialize;
use serde_json::json;

use crate::common::{
  ApiResponse, get_with_bearer, patch_json_with_bearer, post_json_with_bearer, read_json,
  register_user, test_app,
};

#[derive(Deserialize)]
struct OauthProviderView {
  id: i64,
  site_id: Option<i64>,
  enabled: bool,
  provider_code: String,
  client_id: String,
  #[allow(dead_code)]
  redirect_uri: String,
}

#[tokio::test]
async fn admin_can_create_list_and_update_oauth_provider() {
  let app = test_app().await;
  let admin = register_user(&app, "oauth-admin@example.com", "secret123")
    .await
    .data
    .expect("admin");

  let created = post_json_with_bearer(
    &app,
    "/api/admin/oauth/providers",
    &admin.token,
    json!({
      "provider_code": "github",
      "client_id": "client-id",
      "client_secret": "client-secret",
      "redirect_uri": "http://localhost:7410/api/auth/oauth/github/callback"
    }),
  )
  .await;
  assert_eq!(created.status(), StatusCode::OK);
  let created_body: ApiResponse<OauthProviderView> = read_json(created).await;
  let provider = created_body.data.expect("created oauth provider");
  assert_eq!(provider.provider_code, "github");
  assert!(provider.enabled);
  assert_eq!(provider.client_id, "client-id");
  assert!(provider.site_id.is_none());

  let listed = get_with_bearer(&app, "/api/admin/oauth/providers", &admin.token).await;
  assert_eq!(listed.status(), StatusCode::OK);
  let listed_body: ApiResponse<Vec<OauthProviderView>> = read_json(listed).await;
  assert_eq!(listed_body.data.expect("oauth providers").len(), 1);

  let updated = patch_json_with_bearer(
    &app,
    &format!("/api/admin/oauth/providers/{}", provider.id),
    &admin.token,
    json!({ "enabled": false }),
  )
  .await;
  assert_eq!(updated.status(), StatusCode::OK);
  let updated_body: ApiResponse<OauthProviderView> = read_json(updated).await;
  assert!(!updated_body.data.expect("updated oauth provider").enabled);
}

#[tokio::test]
async fn oauth_start_uses_enabled_provider_from_db() {
  let app = test_app().await;
  let admin = register_user(&app, "oauth-start@example.com", "secret123")
    .await
    .data
    .expect("admin");

  let missing = crate::common::request(
    &app,
    crate::common::build_request(
      "GET",
      "/api/auth/oauth/github/start",
      axum::body::Body::empty(),
    ),
  )
  .await;
  assert_eq!(missing.status(), StatusCode::BAD_REQUEST);

  let _ = post_json_with_bearer(
    &app,
    "/api/admin/oauth/providers",
    &admin.token,
    json!({
      "provider_code": "github",
      "client_id": "client-id",
      "client_secret": "client-secret",
      "redirect_uri": "http://localhost:7410/api/auth/oauth/github/callback"
    }),
  )
  .await;

  let started = crate::common::request(
    &app,
    crate::common::build_request(
      "GET",
      "/api/auth/oauth/github/start",
      axum::body::Body::empty(),
    ),
  )
  .await;
  assert_eq!(started.status(), StatusCode::OK);
  let started_body: ApiResponse<serde_json::Value> = read_json(started).await;
  let auth_url = started_body
    .data
    .expect("oauth start")
    .get("auth_url")
    .and_then(|value| value.as_str())
    .expect("auth_url")
    .to_string();
  assert!(auth_url.contains("client_id=client-id"));
  assert!(auth_url.contains("github.com/login/oauth/authorize"));
}
