use std::collections::BTreeMap;

use axum::http::StatusCode;
use serde::Deserialize;
use serde_json::{Value, json};

use crate::common::{
  ApiResponse, create_site, get_with_bearer, post_json, post_json_with_bearer, read_json,
  register_user, test_app,
};

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
