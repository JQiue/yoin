use axum::http::StatusCode;
use serde::Deserialize;
use serde_json::{Value, json};
use yoin::entity::sites::SiteConfig;

use crate::common::{
  ApiResponse, get_with_bearer, patch_json_with_bearer, post_json_with_bearer, read_json,
  register_user, request, test_app,
};

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
  assert_eq!(body.code, "ok");
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
  assert_eq!(body.code, "forbidden");
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
  assert_eq!(body.code, "forbidden");
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

#[derive(Deserialize)]
struct PublicSiteConfigView {
  allow_anonymous: bool,
  allow_private: bool,
  max_comment_length: usize,
  comment_limit_seconds: i64,
  allowed_reactions: Vec<String>,
}

#[tokio::test]
async fn guest_can_read_public_site_config() {
  let app = test_app().await;
  let admin = register_user(&app, "admin-public-config@example.com", "secret123")
    .await
    .data
    .expect("admin");

  let created = post_json_with_bearer(
    &app,
    "/api/sites",
    &admin.token,
    json!({
      "name": "PublicConfig",
      "url": "https://example.com",
      "config": {
        "allow_anonymous": true,
        "max_comment_length": 2048,
        "comment_limit_seconds": 10
      }
    }),
  )
  .await;
  let created_body: ApiResponse<SiteView> = read_json(created).await;
  let site = created_body.data.expect("created site");

  let resp = request(
    &app,
    crate::common::build_request(
      "GET",
      &format!("/api/sites/{}/config", site.id),
      axum::body::Body::empty(),
    ),
  )
  .await;
  assert_eq!(resp.status(), StatusCode::OK);

  let body: ApiResponse<PublicSiteConfigView> = read_json(resp).await;
  let config = body.data.expect("public config");
  assert!(config.allow_anonymous);
  assert!(config.allow_private);
  assert_eq!(config.max_comment_length, 2048);
  assert_eq!(config.comment_limit_seconds, 10);
  assert_eq!(config.allowed_reactions, vec!["👍", "❤️", "😄", "🎉", "👎"]);
}

#[tokio::test]
async fn admin_can_update_private_and_reaction_config() {
  let app = test_app().await;
  let admin = register_user(&app, "admin-site-config@example.com", "secret123")
    .await
    .data
    .expect("admin");

  let created = post_json_with_bearer(
    &app,
    "/api/sites",
    &admin.token,
    json!({
      "name": "ConfigSite",
      "url": "https://config.example.com",
      "config": {
        "allow_anonymous": false,
        "allow_private": false,
        "max_comment_length": 512,
        "comment_limit_seconds": 0,
        "allowed_reactions": ["👍", "👎"]
      }
    }),
  )
  .await;
  assert_eq!(created.status(), StatusCode::OK);
  let created_body: ApiResponse<SiteView> = read_json(created).await;
  let site = created_body.data.expect("created site");
  assert!(!site.config.allow_private);
  assert_eq!(site.config.allowed_reactions, vec!["👍", "👎"]);

  let resp = request(
    &app,
    crate::common::build_request(
      "GET",
      &format!("/api/sites/{}/config", site.id),
      axum::body::Body::empty(),
    ),
  )
  .await;
  assert_eq!(resp.status(), StatusCode::OK);
  let body: ApiResponse<PublicSiteConfigView> = read_json(resp).await;
  let config = body.data.expect("public config");
  assert!(!config.allow_anonymous);
  assert!(!config.allow_private);
  assert_eq!(config.allowed_reactions, vec!["👍", "👎"]);
}

#[tokio::test]
async fn admin_cannot_set_unknown_reaction() {
  let app = test_app().await;
  let admin = register_user(&app, "admin-bad-reaction@example.com", "secret123")
    .await
    .data
    .expect("admin");

  let resp = post_json_with_bearer(
    &app,
    "/api/sites",
    &admin.token,
    json!({
      "name": "BadReaction",
      "url": "https://bad.example.com",
      "config": {
        "allow_anonymous": false,
        "max_comment_length": 1024,
        "comment_limit_seconds": 0,
        "allowed_reactions": ["🔥"]
      }
    }),
  )
  .await;
  assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}
