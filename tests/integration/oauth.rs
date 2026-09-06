use axum::http::StatusCode;
use serde::Deserialize;
use serde_json::json;
use yoin::helper::OauthProfile;

use crate::common::{
  ApiResponse, TEST_JWT_KEY, get_with_bearer, patch_json_with_bearer, post_json_with_bearer,
  read_json, register_user, test_app, test_service,
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

#[tokio::test]
async fn public_oauth_providers_omit_secrets_and_disabled_entries() {
  let app = test_app().await;
  let admin = register_user(&app, "oauth-public@example.com", "secret123")
    .await
    .data
    .expect("admin");

  let empty = crate::common::request(
    &app,
    crate::common::build_request(
      "GET",
      "/api/auth/oauth/providers",
      axum::body::Body::empty(),
    ),
  )
  .await;
  assert_eq!(empty.status(), StatusCode::OK);
  let empty_body: ApiResponse<Vec<serde_json::Value>> = read_json(empty).await;
  assert!(empty_body.data.expect("providers").is_empty());

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
  let _ = post_json_with_bearer(
    &app,
    "/api/admin/oauth/providers",
    &admin.token,
    json!({
      "provider_code": "qq",
      "enabled": false,
      "client_id": "qq-id",
      "client_secret": "qq-secret",
      "redirect_uri": "http://localhost:7410/api/auth/oauth/qq/callback"
    }),
  )
  .await;

  let listed = crate::common::request(
    &app,
    crate::common::build_request(
      "GET",
      "/api/auth/oauth/providers",
      axum::body::Body::empty(),
    ),
  )
  .await;
  assert_eq!(listed.status(), StatusCode::OK);
  let listed_body: ApiResponse<Vec<serde_json::Value>> = read_json(listed).await;
  let providers = listed_body.data.expect("providers");
  assert_eq!(providers.len(), 1);
  assert_eq!(providers[0]["provider_code"], "github");
  assert!(providers[0].get("client_secret").is_none());
  assert!(providers[0].get("client_id").is_none());
}

#[tokio::test]
async fn oauth_login_creates_user_and_reuses_identity() {
  let service = test_service().await;
  let profile = OauthProfile {
    id: "42".to_string(),
    nickname: "octocat".to_string(),
    avatar: "https://example.com/octocat.png".to_string(),
    email: None,
  };

  let first = service
    .login_or_register_oauth_user("github", profile.clone(), TEST_JWT_KEY)
    .await
    .expect("oauth register");
  assert_eq!(first.user.nickname, "octocat");
  assert_eq!(first.user.email, "github+42@oauth.yoin.local");
  assert!(!first.token.is_empty());

  let second = service
    .login_or_register_oauth_user("github", profile, TEST_JWT_KEY)
    .await
    .expect("oauth login");
  assert_eq!(second.user.email, first.user.email);
  assert_eq!(second.user.nickname, first.user.nickname);
}

#[tokio::test]
async fn oauth_login_binds_existing_email() {
  let service = test_service().await;
  let existing = service
    .create_user(
      "tester".to_string(),
      "".to_string(),
      "octocat@example.com".to_string(),
      "secret123".to_string(),
      TEST_JWT_KEY,
    )
    .await
    .expect("existing user");

  let linked = service
    .login_or_register_oauth_user(
      "github",
      OauthProfile {
        id: "99".to_string(),
        nickname: "octocat".to_string(),
        avatar: "https://example.com/octocat.png".to_string(),
        email: Some("octocat@example.com".to_string()),
      },
      TEST_JWT_KEY,
    )
    .await
    .expect("oauth bind");
  assert_eq!(linked.user.email, existing.user.email);
  assert_eq!(linked.user.nickname, existing.user.nickname);
}
