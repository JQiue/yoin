use axum::http::StatusCode;
use serde::Deserialize;
use serde_json::json;

use crate::common::{
  ApiResponse, SiteView, get_with_bearer, patch_json_with_bearer, post_json_with_bearer, read_json,
  register_user, test_app,
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
