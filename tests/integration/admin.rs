use axum::http::StatusCode;
use serde::Deserialize;
use serde_json::json;

use crate::common::{
  ApiResponse, create_site, delete_with_bearer, get_with_bearer, patch_json_with_bearer,
  post_json_with_bearer, read_json, register_user, test_app,
};

#[derive(Deserialize)]
struct UserView {
  id: i64,
  nickname: String,
  email: String,
  role_bindings: Vec<BindingSummary>,
}

#[derive(Deserialize)]
struct BindingSummary {
  role_name: Option<String>,
  scope_type: String,
}

#[derive(Deserialize)]
struct RoleView {
  id: i64,
  name: String,
  permission_names: Vec<String>,
}

#[derive(Deserialize)]
struct UserRoleBindingView {
  id: i64,
  user_id: i64,
  role_id: i64,
  role_name: Option<String>,
  scope_type: String,
  scope_id: Option<String>,
}

#[tokio::test]
async fn admin_can_list_users() {
  let app = test_app().await;
  let admin = register_user(&app, "admin-users@example.com", "secret123")
    .await
    .data
    .expect("admin");
  let _listed = register_user(&app, "listed-user@example.com", "secret123")
    .await
    .data
    .expect("user");

  let resp = get_with_bearer(&app, "/api/admin/users", &admin.token).await;
  assert_eq!(resp.status(), StatusCode::OK);
  let body: ApiResponse<Vec<UserView>> = read_json(resp).await;
  let users = body.data.expect("users");
  assert!(
    users
      .iter()
      .any(|item| item.email == "admin-users@example.com")
  );
  let listed = users
    .iter()
    .find(|item| item.email == "listed-user@example.com")
    .expect("listed user");
  assert_eq!(listed.nickname, "tester");
  assert!(listed.role_bindings.is_empty());

  let admin_user = users
    .iter()
    .find(|item| item.email == "admin-users@example.com")
    .expect("admin user");
  assert!(
    admin_user
      .role_bindings
      .iter()
      .any(
        |binding| binding.role_name.as_deref() == Some("super_admin")
          && binding.scope_type == "global"
      )
  );
}

#[tokio::test]
async fn admin_can_edit_role_permissions_and_bindings() {
  let app = test_app().await;
  let admin = register_user(&app, "admin-rbac@example.com", "secret123")
    .await
    .data
    .expect("admin");
  let _user = register_user(&app, "bound-user@example.com", "secret123")
    .await
    .data
    .expect("user");
  let site = create_site(&app, &admin.token, "RbacSite", true, 0)
    .await
    .data
    .expect("site");
  let users_resp = get_with_bearer(&app, "/api/admin/users", &admin.token).await;
  let users: ApiResponse<Vec<UserView>> = read_json(users_resp).await;
  let bound_user = users
    .data
    .expect("users")
    .into_iter()
    .find(|item| item.email == "bound-user@example.com")
    .expect("bound user");

  let roles_resp = get_with_bearer(&app, "/api/admin/rbac/roles", &admin.token).await;
  let roles: ApiResponse<Vec<RoleView>> = read_json(roles_resp).await;
  let roles = roles.data.expect("roles");
  let moderator = roles
    .iter()
    .find(|role| role.name == "moderator")
    .expect("moderator");
  let site_admin = roles
    .iter()
    .find(|role| role.name == "site_admin")
    .expect("site_admin");

  let updated = patch_json_with_bearer(
    &app,
    &format!("/api/admin/rbac/roles/{}/permissions", moderator.id),
    &admin.token,
    json!({
      "permission_names": ["comment.moderate"]
    }),
  )
  .await;
  assert_eq!(updated.status(), StatusCode::OK);
  let updated_role: ApiResponse<RoleView> = read_json(updated).await;
  let updated_role = updated_role.data.expect("updated role");
  assert_eq!(updated_role.permission_names, vec!["comment.moderate"]);

  let created = post_json_with_bearer(
    &app,
    "/api/admin/rbac/user-role-bindings",
    &admin.token,
    json!({
      "user_id": bound_user.id,
      "role_id": site_admin.id,
      "scope_type": "site",
      "scope_id": site.id.to_string()
    }),
  )
  .await;
  assert_eq!(created.status(), StatusCode::OK);
  let binding: ApiResponse<UserRoleBindingView> = read_json(created).await;
  let binding = binding.data.expect("binding");
  assert_eq!(binding.user_id, bound_user.id);
  assert_eq!(binding.role_id, site_admin.id);
  assert_eq!(binding.scope_type, "site");
  assert_eq!(
    binding.scope_id.as_deref(),
    Some(site.id.to_string().as_str())
  );

  let deleted = delete_with_bearer(
    &app,
    &format!("/api/admin/rbac/user-role-bindings/{}", binding.id),
    &admin.token,
  )
  .await;
  assert_eq!(deleted.status(), StatusCode::OK);

  let listed = get_with_bearer(&app, "/api/admin/rbac/user-role-bindings", &admin.token).await;
  let listed: ApiResponse<Vec<UserRoleBindingView>> = read_json(listed).await;
  let listed = listed.data.expect("bindings");
  assert!(!listed.iter().any(|item| item.id == binding.id));
}

#[tokio::test]
async fn cannot_remove_last_global_super_admin() {
  let app = test_app().await;
  let admin = register_user(&app, "admin-last-super@example.com", "secret123")
    .await
    .data
    .expect("admin");

  let listed = get_with_bearer(&app, "/api/admin/rbac/user-role-bindings", &admin.token).await;
  let listed: ApiResponse<Vec<UserRoleBindingView>> = read_json(listed).await;
  let listed = listed.data.expect("bindings");
  let super_binding = listed
    .iter()
    .find(|binding| {
      binding.role_name.as_deref() == Some("super_admin") && binding.scope_type == "global"
    })
    .expect("super admin binding");

  let resp = delete_with_bearer(
    &app,
    &format!("/api/admin/rbac/user-role-bindings/{}", super_binding.id),
    &admin.token,
  )
  .await;
  assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}
