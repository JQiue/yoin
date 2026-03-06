mod common;

use axum::http::StatusCode;
use serde::Deserialize;
use serde_json::{Value, json};

use common::{
  ApiResponse, get_with_bearer, patch_json_with_bearer, post_json_with_bearer, read_json,
  register_user, test_app,
};
use yoin::entity::sites::SiteConfig;

#[derive(Deserialize)]
struct SiteView {
  id: i64,
  name: String,
  url: String,
  config: SiteConfig,
}

#[tokio::test]
async fn admin_can_create_site() {
  let app = test_app().await;
  let admin = register_user(&app, "admin@example.com", "secret123")
    .await
    .data
    .expect("admin");

  let resp = post_json_with_bearer(
    &app,
    "/api/sites",
    &admin.token,
    json!({
      "name": "Docs",
      "url": "https://example.com",
      "config": {
        "allow_anonymous": true,
        "max_comment_length": 2048,
        "comment_limit_seconds": 10
      }
    }),
  )
  .await;
  assert_eq!(resp.status(), StatusCode::OK);

  let body: ApiResponse<SiteView> = read_json(resp).await;
  assert_eq!(body.code, 0);
  let site = body.data.expect("site data");
  assert_eq!(site.name, "Docs");
  assert_eq!(site.url, "https://example.com");
  assert!(site.config.allow_anonymous);
}

#[tokio::test]
async fn admin_can_update_site() {
  let app = test_app().await;
  let admin = register_user(&app, "admin2@example.com", "secret123")
    .await
    .data
    .expect("admin");

  let created = post_json_with_bearer(
    &app,
    "/api/sites",
    &admin.token,
    json!({
      "name": "Docs",
      "url": "https://example.com",
      "config": {
        "allow_anonymous": false,
        "max_comment_length": 1024,
        "comment_limit_seconds": 60
      }
    }),
  )
  .await;
  let created_body: ApiResponse<SiteView> = read_json(created).await;
  let site = created_body.data.expect("created site");

  let resp = patch_json_with_bearer(
    &app,
    "/api/sites",
    &admin.token,
    json!({
      "id": site.id,
      "name": "Docs Updated",
      "url": "https://new.example.com",
      "config": {
        "allow_anonymous": true,
        "max_comment_length": 4096,
        "comment_limit_seconds": 5
      }
    }),
  )
  .await;
  assert_eq!(resp.status(), StatusCode::OK);

  let body: ApiResponse<SiteView> = read_json(resp).await;
  let updated = body.data.expect("updated site");
  assert_eq!(updated.name, "Docs Updated");
  assert_eq!(updated.url, "https://new.example.com");
  assert_eq!(updated.config.max_comment_length, 4096);
  assert_eq!(updated.config.comment_limit_seconds, 5);
  assert!(updated.config.allow_anonymous);
}

#[tokio::test]
async fn normal_user_cannot_create_site() {
  let app = test_app().await;
  let _ = register_user(&app, "admin3@example.com", "secret123").await;
  let user = register_user(&app, "user@example.com", "secret123")
    .await
    .data
    .expect("normal user");

  let resp = post_json_with_bearer(
    &app,
    "/api/sites",
    &user.token,
    json!({
      "name": "Blocked",
      "url": "https://example.com",
      "config": {
        "allow_anonymous": false,
        "max_comment_length": 1024,
        "comment_limit_seconds": 60
      }
    }),
  )
  .await;
  assert_eq!(resp.status(), StatusCode::FORBIDDEN);

  let body: ApiResponse<Value> = read_json(resp).await;
  assert_ne!(body.code, 0);
}

#[tokio::test]
async fn normal_user_cannot_update_site() {
  let app = test_app().await;
  let admin = register_user(&app, "admin4@example.com", "secret123")
    .await
    .data
    .expect("admin");
  let user = register_user(&app, "user2@example.com", "secret123")
    .await
    .data
    .expect("normal user");

  let created = post_json_with_bearer(
    &app,
    "/api/sites",
    &admin.token,
    json!({
      "name": "Docs",
      "url": "https://example.com",
      "config": {
        "allow_anonymous": false,
        "max_comment_length": 1024,
        "comment_limit_seconds": 60
      }
    }),
  )
  .await;
  let created_body: ApiResponse<SiteView> = read_json(created).await;
  let site = created_body.data.expect("created site");

  let resp = patch_json_with_bearer(
    &app,
    "/api/sites",
    &user.token,
    json!({
      "id": site.id,
      "name": "Should Fail"
    }),
  )
  .await;
  assert_eq!(resp.status(), StatusCode::FORBIDDEN);

  let body: ApiResponse<Value> = read_json(resp).await;
  assert_ne!(body.code, 0);
}

#[tokio::test]
async fn admin_can_list_sites() {
  let app = test_app().await;
  let admin = register_user(&app, "admin5@example.com", "secret123")
    .await
    .data
    .expect("admin");

  let resp = get_with_bearer(&app, "/api/sites", &admin.token).await;
  assert_eq!(resp.status(), StatusCode::OK);

  let body: ApiResponse<Vec<SiteView>> = read_json(resp).await;
  let sites = body.data.expect("sites");
  assert!(!sites.is_empty());
}
