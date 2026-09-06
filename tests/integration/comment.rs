use axum::{
  body::Body,
  http::{StatusCode, header},
  response::Response,
};
use serde::Deserialize;
use serde_json::{Value, json};

use crate::common::{
  self, ApiResponse, create_site, post_json, post_json_with_bearer, read_json, register_user,
  test_app,
};

#[derive(Deserialize)]
struct CommentView {
  id: i64,
  thread_id: Option<i64>,
  parent_id: Option<i64>,
  nickname: String,
  website: String,
  avatar: String,
  content: String,
  is_anonymous: bool,
  is_private: bool,
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
async fn guest_comment_allowed_when_anonymous_mode_is_disabled() {
  let app = test_app().await;
  let admin = register_user(&app, "admin-comment@example.com", "secret123")
    .await
    .data
    .expect("admin");
  let site = create_site(&app, &admin.token, "CommentsOpen", false, 0)
    .await
    .data
    .expect("site");

  let resp = post_json(
    &app,
    "/api/comments",
    comment_payload(site.id, "hello world", None),
  )
  .await;
  assert_eq!(resp.status(), StatusCode::OK);

  let body: ApiResponse<CommentView> = read_json(resp).await;
  let comment = body.data.expect("comment");
  assert_eq!(comment.content, "<p>hello world</p>\n");
  assert_eq!(comment.nickname, "guest");
  assert_eq!(comment.thread_id, None);
  assert!(!comment.is_anonymous);
}

#[tokio::test]
async fn create_comment_returns_html_content() {
  let app = test_app().await;
  let admin = register_user(&app, "admin-markdown@example.com", "secret123")
    .await
    .data
    .expect("admin");
  let site = create_site(&app, &admin.token, "MarkdownComments", true, 0)
    .await
    .data
    .expect("site");

  let resp = post_json(
    &app,
    "/api/comments",
    comment_payload(site.id, "**hello** from markdown", None),
  )
  .await;
  assert_eq!(resp.status(), StatusCode::OK);

  let body: ApiResponse<CommentView> = read_json(resp).await;
  let comment = body.data.expect("comment");
  assert!(comment.content.contains("<strong>hello</strong>"));
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

  let guest_resp = post_json(
    &app,
    "/api/comments",
    comment_payload(site.id, "guest still allowed", None),
  )
  .await;
  assert_eq!(guest_resp.status(), StatusCode::OK);

  let resp = post_json(
    &app,
    "/api/comments",
    comment_payload_with_flags(site.id, "blocked", None, true, false),
  )
  .await;
  assert_eq!(resp.status(), StatusCode::FORBIDDEN);

  let body: ApiResponse<Value> = read_json(resp).await;
  assert_eq!(body.code, "forbidden");
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
  assert_eq!(comment.content, "<p>member comment</p>\n");
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

  let root_resp = post_json(
    &app,
    "/api/comments",
    comment_payload(site.id, "root comment", None),
  )
  .await;
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
async fn reply_parent_must_belong_to_same_site_and_page() {
  let app = test_app().await;
  let admin = register_user(&app, "admin-cross-site@example.com", "secret123")
    .await
    .data
    .expect("admin");
  let site_a = create_site(&app, &admin.token, "SiteA", true, 0)
    .await
    .data
    .expect("site a");
  let site_b = create_site(&app, &admin.token, "SiteB", true, 0)
    .await
    .data
    .expect("site b");

  let root_resp = post_json(
    &app,
    "/api/comments",
    comment_payload(site_a.id, "root comment", None),
  )
  .await;
  let root_body: ApiResponse<CommentView> = read_json(root_resp).await;
  let root = root_body.data.expect("root");

  let resp = post_json(
    &app,
    "/api/comments",
    comment_payload(site_b.id, "invalid cross site reply", Some(root.id)),
  )
  .await;
  assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
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

  let root_resp = post_json(
    &app,
    "/api/comments",
    comment_payload(site.id, "root comment", None),
  )
  .await;
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

fn comment_payload_with_flags(
  site_id: i64,
  content: &str,
  parent_id: Option<i64>,
  is_anonymous: bool,
  is_private: bool,
) -> Value {
  json!({
    "site_id": site_id,
    "nickname": "guest",
    "website": "https://guest.example.com",
    "content": content,
    "page_path": "/post/hello",
    "email": "guest@example.com",
    "parent_id": parent_id,
    "is_anonymous": is_anonymous,
    "is_private": is_private
  })
}

async fn post_json_with_guest(
  app: &axum::Router,
  uri: &str,
  guest_id: &str,
  payload: Value,
) -> Response {
  let mut req = common::build_request("POST", uri, Body::from(payload.to_string()));
  req
    .headers_mut()
    .insert(header::CONTENT_TYPE, "application/json".parse().unwrap());
  req
    .headers_mut()
    .insert("x-yoin-guest-id", guest_id.parse().unwrap());
  common::request(app, req).await
}

async fn list_comments(
  app: &axum::Router,
  site_id: i64,
  token: Option<&str>,
) -> PageResponse<CommentView> {
  list_comments_as(app, site_id, token, None).await
}

async fn list_comments_as(
  app: &axum::Router,
  site_id: i64,
  token: Option<&str>,
  guest_id: Option<&str>,
) -> PageResponse<CommentView> {
  let uri = format!(
    "/api/comments?site_id={}&page_path=%2Fpost%2Fhello&page_size=10&page_offset=1&sort=created_desc",
    site_id
  );
  let resp = if let Some(token) = token {
    common::get_with_bearer(app, &uri, token).await
  } else {
    let mut req = common::build_request("GET", &uri, Body::empty());
    if let Some(guest_id) = guest_id {
      req
        .headers_mut()
        .insert("x-yoin-guest-id", guest_id.parse().unwrap());
    }
    common::request(app, req).await
  };
  assert_eq!(resp.status(), StatusCode::OK);
  let body: ApiResponse<PageResponse<CommentView>> = read_json(resp).await;
  body.data.expect("page")
}

#[tokio::test]
async fn anonymous_comment_hides_identity_from_guests() {
  let app = test_app().await;
  let admin = register_user(&app, "admin-anonymous@example.com", "secret123")
    .await
    .data
    .expect("admin");
  let site = create_site(&app, &admin.token, "AnonymousComments", true, 0)
    .await
    .data
    .expect("site");
  let user = register_user(&app, "anon-member@example.com", "secret123")
    .await
    .data
    .expect("user");

  let resp = post_json_with_bearer(
    &app,
    "/api/comments",
    &user.token,
    comment_payload_with_flags(site.id, "hidden identity", None, true, false),
  )
  .await;
  assert_eq!(resp.status(), StatusCode::OK);
  let created: ApiResponse<CommentView> = read_json(resp).await;
  let created = created.data.expect("comment");
  assert!(created.is_anonymous);
  assert_eq!(created.nickname, "匿名");
  assert_eq!(created.website, "");
  assert_eq!(created.avatar, "");

  let guest_page = list_comments(&app, site.id, None).await;
  assert_eq!(guest_page.total, 1);
  assert_eq!(guest_page.items[0].nickname, "匿名");
  assert_eq!(guest_page.items[0].website, "");
  assert_eq!(guest_page.items[0].avatar, "");
  assert!(guest_page.items[0].is_anonymous);

  let admin_page = list_comments(&app, site.id, Some(&admin.token)).await;
  assert_eq!(admin_page.total, 1);
  assert_eq!(admin_page.items[0].nickname, "tester");
  assert!(admin_page.items[0].is_anonymous);
}

#[tokio::test]
async fn private_comment_is_hidden_from_other_guests() {
  let app = test_app().await;
  let admin = register_user(&app, "admin-private@example.com", "secret123")
    .await
    .data
    .expect("admin");
  let site = create_site(&app, &admin.token, "PrivateComments", true, 0)
    .await
    .data
    .expect("site");

  let author_guest = "guest-author-aaaaaaaa";
  let other_guest = "guest-other-bbbbbbbb";
  let resp = post_json_with_guest(
    &app,
    "/api/comments",
    author_guest,
    comment_payload_with_flags(site.id, "only owner", None, false, true),
  )
  .await;
  assert_eq!(resp.status(), StatusCode::OK);
  let created: ApiResponse<CommentView> = read_json(resp).await;
  let created = created.data.expect("comment");
  assert!(created.is_private);

  let author_page = list_comments_as(&app, site.id, None, Some(author_guest)).await;
  assert_eq!(author_page.total, 1);
  assert_eq!(author_page.items[0].id, created.id);
  assert!(author_page.items[0].is_private);

  let other_page = list_comments_as(&app, site.id, None, Some(other_guest)).await;
  assert_eq!(other_page.total, 0);
  assert!(other_page.items.is_empty());

  let stranger_page = list_comments(&app, site.id, None).await;
  assert_eq!(stranger_page.total, 0);
  assert!(stranger_page.items.is_empty());

  let admin_page = list_comments(&app, site.id, Some(&admin.token)).await;
  assert_eq!(admin_page.total, 1);
  assert_eq!(admin_page.items[0].id, created.id);
  assert!(admin_page.items[0].is_private);

  let reply_resp = post_json_with_guest(
    &app,
    "/api/comments",
    other_guest,
    comment_payload(site.id, "cannot see parent", Some(created.id)),
  )
  .await;
  assert_eq!(reply_resp.status(), StatusCode::NOT_FOUND);

  let author_reply = post_json_with_guest(
    &app,
    "/api/comments",
    author_guest,
    comment_payload(site.id, "author can reply", Some(created.id)),
  )
  .await;
  assert_eq!(author_reply.status(), StatusCode::OK);
}

#[tokio::test]
async fn private_comment_rejected_when_site_disables_it() {
  let app = test_app().await;
  let admin = register_user(&app, "admin-no-private@example.com", "secret123")
    .await
    .data
    .expect("admin");
  let created = post_json_with_bearer(
    &app,
    "/api/sites",
    &admin.token,
    json!({
      "name": "NoPrivate",
      "url": "https://noprivate.example.com",
      "config": {
        "allow_anonymous": true,
        "allow_private": false,
        "max_comment_length": 1024,
        "comment_limit_seconds": 0
      }
    }),
  )
  .await;
  let site = read_json::<ApiResponse<crate::common::SiteView>>(created)
    .await
    .data
    .expect("site");

  let resp = post_json(
    &app,
    "/api/comments",
    comment_payload_with_flags(site.id, "secret", None, false, true),
  )
  .await;
  assert_eq!(resp.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn logged_in_user_can_see_own_private_comment() {
  let app = test_app().await;
  let admin = register_user(&app, "admin-private-user@example.com", "secret123")
    .await
    .data
    .expect("admin");
  let site = create_site(&app, &admin.token, "PrivateUserComments", true, 0)
    .await
    .data
    .expect("site");
  let user = register_user(&app, "private-author@example.com", "secret123")
    .await
    .data
    .expect("user");
  let other = register_user(&app, "private-stranger@example.com", "secret123")
    .await
    .data
    .expect("other");

  let resp = post_json_with_bearer(
    &app,
    "/api/comments",
    &user.token,
    comment_payload_with_flags(site.id, "mine", None, false, true),
  )
  .await;
  assert_eq!(resp.status(), StatusCode::OK);
  let created: ApiResponse<CommentView> = read_json(resp).await;
  let created = created.data.expect("comment");

  let author_page = list_comments(&app, site.id, Some(&user.token)).await;
  assert_eq!(author_page.total, 1);
  assert_eq!(author_page.items[0].id, created.id);

  let other_page = list_comments(&app, site.id, Some(&other.token)).await;
  assert_eq!(other_page.total, 0);
}
