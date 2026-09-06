use axum::http::StatusCode;
use serde::Deserialize;
use serde_json::json;

use crate::common::{
  ApiResponse, SiteView, get_with_bearer, patch_json_with_bearer, post_json, post_json_with_bearer,
  read_json, register_user, test_app,
};

#[derive(Deserialize)]
struct ModerationProviderView {
  id: i64,
  site_id: i64,
  provider_kind: String,
  enabled: bool,
}

#[tokio::test]
async fn admin_can_create_list_and_update_moderation_provider() {
  let app = test_app().await;
  let admin = register_user(&app, "moderation-admin@example.com", "secret123")
    .await
    .data
    .expect("admin");

  let created_site = post_json_with_bearer(
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
  let site: SiteView = read_json::<ApiResponse<SiteView>>(created_site)
    .await
    .data
    .expect("site");

  let created = post_json_with_bearer(
    &app,
    "/api/admin/moderation/providers",
    &admin.token,
    json!({
      "site_id": site.id,
      "provider_kind": "llm",
      "enabled": true,
      "config": {
        "model": "gpt-4o-mini",
        "api_base": "https://api.example.com/v1",
        "api_key": "sk-test",
        "rule": "reject spam"
      }
    }),
  )
  .await;
  assert_eq!(created.status(), StatusCode::OK);
  let created_body: ApiResponse<ModerationProviderView> = read_json(created).await;
  let provider = created_body.data.expect("created moderation provider");
  assert_eq!(provider.site_id, site.id);
  assert_eq!(provider.provider_kind, "llm");
  assert!(provider.enabled);

  let listed = get_with_bearer(&app, "/api/admin/moderation/providers", &admin.token).await;
  assert_eq!(listed.status(), StatusCode::OK);
  let listed_body: ApiResponse<Vec<ModerationProviderView>> = read_json(listed).await;
  assert_eq!(listed_body.data.expect("moderation providers").len(), 1);

  let updated = patch_json_with_bearer(
    &app,
    &format!("/api/admin/moderation/providers/{}", provider.id),
    &admin.token,
    json!({ "enabled": false }),
  )
  .await;
  assert_eq!(updated.status(), StatusCode::OK);
  let updated_body: ApiResponse<ModerationProviderView> = read_json(updated).await;
  assert!(
    !updated_body
      .data
      .expect("updated moderation provider")
      .enabled
  );
}

#[derive(Deserialize)]
struct CommentView {
  id: i64,
  content: String,
}

#[derive(Deserialize)]
struct PageResponse<T> {
  items: Vec<T>,
  total: u64,
}

#[derive(Deserialize)]
struct AdminCommentView {
  id: i64,
  status: String,
}

#[tokio::test]
async fn admin_can_approve_pending_comment() {
  let app = test_app().await;
  let admin = register_user(&app, "moderation-approve@example.com", "secret123")
    .await
    .data
    .expect("admin");
  let guest = register_user(&app, "moderation-guest@example.com", "secret123")
    .await
    .data
    .expect("guest");
  let site = crate::common::create_site(&app, &admin.token, "ModerationQueue", true, 0)
    .await
    .data
    .expect("site");

  let created_provider = post_json_with_bearer(
    &app,
    "/api/admin/moderation/providers",
    &admin.token,
    json!({
      "site_id": site.id,
      "provider_kind": "akismet",
      "enabled": true,
      "config": {
        "api_key": "test",
        "blog_url": "https://example.com"
      }
    }),
  )
  .await;
  assert_eq!(created_provider.status(), StatusCode::OK);

  let created_comment = post_json(
    &app,
    "/api/comments",
    json!({
      "site_id": site.id,
      "nickname": "guest",
      "website": "",
      "content": "please review me",
      "page_path": "/post/hello",
      "email": "guest@example.com",
      "parent_id": null
    }),
  )
  .await;
  assert_eq!(created_comment.status(), StatusCode::OK);
  let comment: CommentView = read_json::<ApiResponse<CommentView>>(created_comment)
    .await
    .data
    .expect("comment");

  let public_before = crate::common::request(
    &app,
    crate::common::build_request(
      "GET",
      &format!(
        "/api/comments?site_id={}&page_path=%2Fpost%2Fhello&page_size=10&page_offset=1&sort=created_desc",
        site.id
      ),
      axum::body::Body::empty(),
    ),
  )
  .await;
  let public_before_body: ApiResponse<PageResponse<CommentView>> = read_json(public_before).await;
  assert_eq!(public_before_body.data.expect("page").total, 0);

  let forbidden = patch_json_with_bearer(
    &app,
    &format!("/api/admin/comments/{}", comment.id),
    &guest.token,
    json!({ "status": "approved" }),
  )
  .await;
  assert_eq!(forbidden.status(), StatusCode::FORBIDDEN);

  let approved = patch_json_with_bearer(
    &app,
    &format!("/api/admin/comments/{}", comment.id),
    &admin.token,
    json!({ "status": "approved" }),
  )
  .await;
  assert_eq!(approved.status(), StatusCode::OK);

  let listed = get_with_bearer(
    &app,
    &format!(
      "/api/admin/comments?site_id={}&page_path=%2Fpost%2Fhello&page_size=10&page_offset=1&sort=created_desc&status=approved",
      site.id
    ),
    &admin.token,
  )
  .await;
  assert_eq!(listed.status(), StatusCode::OK);
  let listed_body: ApiResponse<PageResponse<AdminCommentView>> = read_json(listed).await;
  let page = listed_body.data.expect("admin comments");
  assert_eq!(page.total, 1);
  assert_eq!(page.items[0].id, comment.id);
  assert_eq!(page.items[0].status, "approved");

  let public_after = crate::common::request(
    &app,
    crate::common::build_request(
      "GET",
      &format!(
        "/api/comments?site_id={}&page_path=%2Fpost%2Fhello&page_size=10&page_offset=1&sort=created_desc",
        site.id
      ),
      axum::body::Body::empty(),
    ),
  )
  .await;
  let public_after_body: ApiResponse<PageResponse<CommentView>> = read_json(public_after).await;
  let public_page = public_after_body.data.expect("page");
  assert_eq!(public_page.total, 1);
  assert!(public_page.items[0].content.contains("please review me"));
}
