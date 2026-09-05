use std::collections::BTreeMap;

use axum::{
  body::Body,
  http::{StatusCode, header},
  response::Response,
};
use serde::Deserialize;
use serde_json::{Value, json};

use crate::common::{
  ApiResponse, build_request, create_site, get_with_bearer, post_json, post_json_with_bearer,
  read_json, register_user, request, test_app,
};

async fn get(app: &axum::Router, uri: &str) -> Response {
  request(app, build_request("GET", uri, Body::empty())).await
}

async fn post_json_with_guest(
  app: &axum::Router,
  uri: &str,
  guest_id: &str,
  payload: Value,
) -> Response {
  let mut req = build_request("POST", uri, Body::from(payload.to_string()));
  req
    .headers_mut()
    .insert(header::CONTENT_TYPE, "application/json".parse().unwrap());
  req
    .headers_mut()
    .insert("x-yoin-guest-id", guest_id.parse().unwrap());
  request(app, req).await
}

async fn get_with_guest(app: &axum::Router, uri: &str, guest_id: &str) -> Response {
  let mut req = build_request("GET", uri, Body::empty());
  req
    .headers_mut()
    .insert("x-yoin-guest-id", guest_id.parse().unwrap());
  request(app, req).await
}

#[derive(Deserialize)]
struct CommentView {
  id: i64,
}

#[derive(Deserialize)]
struct ReactionSummaryView {
  counts: BTreeMap<String, u64>,
  my_reaction: Option<String>,
}

fn comment_payload(site_id: i64, content: &str, is_private: bool) -> Value {
  json!({
    "site_id": site_id,
    "nickname": "guest",
    "website": "",
    "content": content,
    "page_path": "/post/hello",
    "email": "guest@example.com",
    "is_private": is_private
  })
}

fn reaction_payload(site_id: i64, comment_id: i64, reaction: &str) -> Value {
  json!({
    "site_id": site_id,
    "target_type": "comment",
    "comment_id": comment_id,
    "page_path": "/post/hello",
    "reaction": reaction
  })
}

#[tokio::test]
async fn user_can_upsert_and_toggle_comment_reaction() {
  let app = test_app().await;
  let admin = register_user(&app, "admin-reaction@example.com", "secret123")
    .await
    .data
    .expect("admin");
  let site = create_site(&app, &admin.token, "ReactionComments", true, 0)
    .await
    .data
    .expect("site");
  let user = register_user(&app, "reactor@example.com", "secret123")
    .await
    .data
    .expect("user");

  let comment_resp = post_json(
    &app,
    "/api/comments",
    comment_payload(site.id, "react me", false),
  )
  .await;
  let comment: ApiResponse<CommentView> = read_json(comment_resp).await;
  let comment = comment.data.expect("comment");

  let resp = post_json_with_bearer(
    &app,
    "/api/reactions",
    &user.token,
    reaction_payload(site.id, comment.id, "👍"),
  )
  .await;
  assert_eq!(resp.status(), StatusCode::OK);
  let body: ApiResponse<ReactionSummaryView> = read_json(resp).await;
  let summary = body.data.expect("summary");
  assert_eq!(summary.counts.get("👍"), Some(&1));
  assert_eq!(summary.my_reaction.as_deref(), Some("👍"));

  let resp = post_json_with_bearer(
    &app,
    "/api/reactions",
    &user.token,
    reaction_payload(site.id, comment.id, "❤️"),
  )
  .await;
  let body: ApiResponse<ReactionSummaryView> = read_json(resp).await;
  let summary = body.data.expect("summary");
  assert_eq!(summary.counts.get("👍"), None);
  assert_eq!(summary.counts.get("❤️"), Some(&1));
  assert_eq!(summary.my_reaction.as_deref(), Some("❤️"));

  let resp = post_json_with_bearer(
    &app,
    "/api/reactions",
    &user.token,
    reaction_payload(site.id, comment.id, "❤️"),
  )
  .await;
  let body: ApiResponse<ReactionSummaryView> = read_json(resp).await;
  let summary = body.data.expect("summary");
  assert!(summary.counts.is_empty());
  assert_eq!(summary.my_reaction, None);
}

#[tokio::test]
async fn guest_cannot_react_to_private_comment() {
  let app = test_app().await;
  let admin = register_user(&app, "admin-private-reaction@example.com", "secret123")
    .await
    .data
    .expect("admin");
  let site = create_site(&app, &admin.token, "PrivateReactions", true, 0)
    .await
    .data
    .expect("site");
  let user = register_user(&app, "nosy@example.com", "secret123")
    .await
    .data
    .expect("user");

  let comment_resp = post_json(
    &app,
    "/api/comments",
    comment_payload(site.id, "secret", true),
  )
  .await;
  let comment: ApiResponse<CommentView> = read_json(comment_resp).await;
  let comment = comment.data.expect("comment");

  let resp = post_json_with_bearer(
    &app,
    "/api/reactions",
    &user.token,
    reaction_payload(site.id, comment.id, "👍"),
  )
  .await;
  assert_eq!(resp.status(), StatusCode::NOT_FOUND);

  let list_uri = format!(
    "/api/reactions?site_id={}&target_type=comment&comment_id={}&page_path=%2Fpost%2Fhello",
    site.id, comment.id
  );
  let list_resp = get_with_bearer(&app, &list_uri, &user.token).await;
  assert_eq!(list_resp.status(), StatusCode::NOT_FOUND);

  let admin_resp = post_json_with_bearer(
    &app,
    "/api/reactions",
    &admin.token,
    reaction_payload(site.id, comment.id, "👍"),
  )
  .await;
  assert_eq!(admin_resp.status(), StatusCode::OK);
}

fn page_reaction_payload(site_id: i64, reaction: &str) -> Value {
  json!({
    "site_id": site_id,
    "target_type": "page",
    "page_path": "/post/hello",
    "reaction": reaction
  })
}

#[derive(Deserialize)]
struct ListedCommentView {
  id: i64,
  reactions: ReactionSummaryView,
}

#[derive(Deserialize)]
struct PageResponse<T> {
  items: Vec<T>,
}

#[tokio::test]
async fn guest_can_upsert_and_toggle_comment_reaction() {
  let app = test_app().await;
  let admin = register_user(&app, "admin-guest-reaction@example.com", "secret123")
    .await
    .data
    .expect("admin");
  let site = create_site(&app, &admin.token, "GuestReactions", true, 0)
    .await
    .data
    .expect("site");

  let comment_resp = post_json(
    &app,
    "/api/comments",
    comment_payload(site.id, "react as guest", false),
  )
  .await;
  let comment: ApiResponse<CommentView> = read_json(comment_resp).await;
  let comment = comment.data.expect("comment");

  let guest_a = "guest-actor-aaaaaaaa";
  let guest_b = "guest-actor-bbbbbbbb";

  let resp = post_json_with_guest(
    &app,
    "/api/reactions",
    guest_a,
    reaction_payload(site.id, comment.id, "👍"),
  )
  .await;
  assert_eq!(resp.status(), StatusCode::OK);
  assert_eq!(
    resp
      .headers()
      .get("x-yoin-guest-id")
      .and_then(|v| v.to_str().ok()),
    Some(guest_a)
  );
  let body: ApiResponse<ReactionSummaryView> = read_json(resp).await;
  let summary = body.data.expect("summary");
  assert_eq!(summary.counts.get("👍"), Some(&1));
  assert_eq!(summary.my_reaction.as_deref(), Some("👍"));

  let resp = post_json_with_guest(
    &app,
    "/api/reactions",
    guest_b,
    reaction_payload(site.id, comment.id, "❤️"),
  )
  .await;
  let body: ApiResponse<ReactionSummaryView> = read_json(resp).await;
  let summary = body.data.expect("summary");
  assert_eq!(summary.counts.get("👍"), Some(&1));
  assert_eq!(summary.counts.get("❤️"), Some(&1));
  assert_eq!(summary.my_reaction.as_deref(), Some("❤️"));

  let resp = post_json_with_guest(
    &app,
    "/api/reactions",
    guest_a,
    reaction_payload(site.id, comment.id, "👍"),
  )
  .await;
  let body: ApiResponse<ReactionSummaryView> = read_json(resp).await;
  let summary = body.data.expect("summary");
  assert_eq!(summary.counts.get("👍"), None);
  assert_eq!(summary.counts.get("❤️"), Some(&1));
  assert_eq!(summary.my_reaction, None);
}

#[tokio::test]
async fn guest_can_react_to_page() {
  let app = test_app().await;
  let admin = register_user(&app, "admin-page-reaction@example.com", "secret123")
    .await
    .data
    .expect("admin");
  let site = create_site(&app, &admin.token, "PageReactions", true, 0)
    .await
    .data
    .expect("site");

  let guest = "guest-page-actor-001";
  let resp = post_json_with_guest(
    &app,
    "/api/reactions",
    guest,
    page_reaction_payload(site.id, "🎉"),
  )
  .await;
  assert_eq!(resp.status(), StatusCode::OK);
  let body: ApiResponse<ReactionSummaryView> = read_json(resp).await;
  let summary = body.data.expect("summary");
  assert_eq!(summary.counts.get("🎉"), Some(&1));
  assert_eq!(summary.my_reaction.as_deref(), Some("🎉"));

  let list_uri = format!(
    "/api/reactions?site_id={}&target_type=page&page_path=%2Fpost%2Fhello",
    site.id
  );
  let list_resp = get_with_guest(&app, &list_uri, guest).await;
  assert_eq!(list_resp.status(), StatusCode::OK);
  let body: ApiResponse<ReactionSummaryView> = read_json(list_resp).await;
  let summary = body.data.expect("summary");
  assert_eq!(summary.counts.get("🎉"), Some(&1));
  assert_eq!(summary.my_reaction.as_deref(), Some("🎉"));
}

#[tokio::test]
async fn comment_list_includes_reaction_summary_for_guest() {
  let app = test_app().await;
  let admin = register_user(&app, "admin-list-reaction@example.com", "secret123")
    .await
    .data
    .expect("admin");
  let site = create_site(&app, &admin.token, "ListReactions", true, 0)
    .await
    .data
    .expect("site");

  let comment_resp = post_json(
    &app,
    "/api/comments",
    comment_payload(site.id, "listed", false),
  )
  .await;
  let comment: ApiResponse<CommentView> = read_json(comment_resp).await;
  let comment = comment.data.expect("comment");

  let guest = "guest-list-actor-001";
  let react_resp = post_json_with_guest(
    &app,
    "/api/reactions",
    guest,
    reaction_payload(site.id, comment.id, "😄"),
  )
  .await;
  assert_eq!(react_resp.status(), StatusCode::OK);

  let list_uri = format!(
    "/api/comments?site_id={}&page_path=%2Fpost%2Fhello&page_offset=1&page_size=10&sort=created_desc",
    site.id
  );
  let list_resp = get_with_guest(&app, &list_uri, guest).await;
  assert_eq!(list_resp.status(), StatusCode::OK);
  let body: ApiResponse<PageResponse<ListedCommentView>> = read_json(list_resp).await;
  let page = body.data.expect("page");
  let listed = page
    .items
    .iter()
    .find(|item| item.id == comment.id)
    .expect("listed comment");
  assert_eq!(listed.reactions.counts.get("😄"), Some(&1));
  assert_eq!(listed.reactions.my_reaction.as_deref(), Some("😄"));
}

#[tokio::test]
async fn missing_guest_id_is_issued_on_reaction_request() {
  let app = test_app().await;
  let admin = register_user(&app, "admin-issued-guest@example.com", "secret123")
    .await
    .data
    .expect("admin");
  let site = create_site(&app, &admin.token, "IssuedGuest", true, 0)
    .await
    .data
    .expect("site");

  let resp = post_json(&app, "/api/reactions", page_reaction_payload(site.id, "👍")).await;
  assert_eq!(resp.status(), StatusCode::OK);
  let issued = resp
    .headers()
    .get("x-yoin-guest-id")
    .and_then(|v| v.to_str().ok())
    .expect("issued guest id");
  assert!(issued.len() >= 8);
  let set_cookie = resp
    .headers()
    .get("set-cookie")
    .and_then(|v| v.to_str().ok())
    .unwrap_or("");
  assert!(set_cookie.contains("yoin_guest_id="));

  let body: ApiResponse<ReactionSummaryView> = read_json(resp).await;
  let summary = body.data.expect("summary");
  assert_eq!(summary.counts.get("👍"), Some(&1));
  assert_eq!(summary.my_reaction.as_deref(), Some("👍"));

  let list_uri = format!(
    "/api/reactions?site_id={}&target_type=page&page_path=%2Fpost%2Fhello",
    site.id
  );
  let list_resp = get(&app, &list_uri).await;
  assert_eq!(list_resp.status(), StatusCode::OK);
}
