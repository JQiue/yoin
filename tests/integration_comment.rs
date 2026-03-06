mod common;

use axum::http::StatusCode;
use serde::Deserialize;
use serde_json::{Value, json};

use common::{ApiResponse, create_site, post_json, post_json_with_bearer, read_json, register_user, test_app};

#[derive(Deserialize)]
struct CommentView {
  id: i64,
  thread_id: Option<i64>,
  parent_id: Option<i64>,
  nickname: String,
  content: String,
}

#[derive(Deserialize)]
struct PageResponse<T> {
  items: Vec<T>,
  page_size: u64,
  page_offset: u64,
  total: u64,
  total_pages: u64,
}

fn comment_payload(site_id: i64, content: &str, parent_id: Option<i64>) -> Value {
  json!({
    "site_id": site_id,
    "nickname": "guest",
    "website": "",
    "content": content,
    "page_path": "/post/hello",
    "email": "guest@example.com",
    "parent_id": parent_id
  })
}

#[tokio::test]
async fn anonymous_comment_allowed_when_site_enables_it() {
  let app = test_app().await;
  let admin = register_user(&app, "admin-comment@example.com", "secret123")
    .await
    .data
    .expect("admin");
  let site = create_site(&app, &admin.token, "CommentsOpen", true, 0)
    .await
    .data
    .expect("site");

  let resp = post_json(&app, "/api/comments", comment_payload(site.id, "hello world", None)).await;
  assert_eq!(resp.status(), StatusCode::OK);

  let body: ApiResponse<CommentView> = read_json(resp).await;
  let comment = body.data.expect("comment");
  assert_eq!(comment.content, "hello world");
  assert_eq!(comment.nickname, "guest");
  assert_eq!(comment.thread_id, None);
}

#[tokio::test]
async fn anonymous_comment_forbidden_when_site_disables_it() {
  let app = test_app().await;
  let admin = register_user(&app, "admin-comment2@example.com", "secret123")
    .await
    .data
    .expect("admin");
  let site = create_site(&app, &admin.token, "CommentsClosed", false, 0)
    .await
    .data
    .expect("site");

  let resp = post_json(&app, "/api/comments", comment_payload(site.id, "blocked", None)).await;
  assert_eq!(resp.status(), StatusCode::FORBIDDEN);

  let body: ApiResponse<Value> = read_json(resp).await;
  assert_ne!(body.code, 0);
}

#[tokio::test]
async fn authenticated_user_comment_uses_profile_nickname() {
  let app = test_app().await;
  let admin = register_user(&app, "admin-comment3@example.com", "secret123")
    .await
    .data
    .expect("admin");
  let site = create_site(&app, &admin.token, "MemberComments", true, 0)
    .await
    .data
    .expect("site");
  let user = register_user(&app, "member@example.com", "secret123")
    .await
    .data
    .expect("user");

  let resp = post_json_with_bearer(
    &app,
    "/api/comments",
    &user.token,
    comment_payload(site.id, "member comment", None),
  )
  .await;
  assert_eq!(resp.status(), StatusCode::OK);

  let body: ApiResponse<CommentView> = read_json(resp).await;
  let comment = body.data.expect("comment");
  assert_eq!(comment.nickname, "tester");
  assert_eq!(comment.content, "member comment");
}

#[tokio::test]
async fn reply_comment_sets_thread_id_and_parent_id() {
  let app = test_app().await;
  let admin = register_user(&app, "admin-comment4@example.com", "secret123")
    .await
    .data
    .expect("admin");
  let site = create_site(&app, &admin.token, "ReplyComments", true, 0)
    .await
    .data
    .expect("site");

  let root_resp =
    post_json(&app, "/api/comments", comment_payload(site.id, "root comment", None)).await;
  let root_body: ApiResponse<CommentView> = read_json(root_resp).await;
  let root = root_body.data.expect("root");

  let reply_resp = post_json(
    &app,
    "/api/comments",
    comment_payload(site.id, "reply comment", Some(root.id)),
  )
  .await;
  assert_eq!(reply_resp.status(), StatusCode::OK);

  let reply_body: ApiResponse<CommentView> = read_json(reply_resp).await;
  let reply = reply_body.data.expect("reply");
  assert_eq!(reply.parent_id, Some(root.id));
  assert_eq!(reply.thread_id, Some(root.id));
}

#[tokio::test]
async fn list_comments_returns_roots_and_replies() {
  let app = test_app().await;
  let admin = register_user(&app, "admin-comment5@example.com", "secret123")
    .await
    .data
    .expect("admin");
  let site = create_site(&app, &admin.token, "ListComments", true, 0)
    .await
    .data
    .expect("site");

  let root_resp =
    post_json(&app, "/api/comments", comment_payload(site.id, "root comment", None)).await;
  let root_body: ApiResponse<CommentView> = read_json(root_resp).await;
  let root = root_body.data.expect("root");

  let _reply_resp = post_json(
    &app,
    "/api/comments",
    comment_payload(site.id, "reply comment", Some(root.id)),
  )
  .await;

  let resp = common::request(
    &app,
    common::build_request(
      "GET",
      &format!(
        "/api/comments?site_id={}&page_path=%2Fpost%2Fhello&page_size=10&page_offset=1&sort=created_desc",
        site.id
      ),
      axum::body::Body::empty(),
    ),
  )
  .await;
  assert_eq!(resp.status(), StatusCode::OK);

  let body: ApiResponse<PageResponse<Value>> = read_json(resp).await;
  let page = body.data.expect("page");
  assert_eq!(page.page_size, 10);
  assert_eq!(page.page_offset, 1);
  assert_eq!(page.total, 1);
  assert_eq!(page.total_pages, 1);
  assert_eq!(page.items.len(), 1);
}
