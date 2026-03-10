use helpers::time::utc_now;
use migration::enums::UserRoleBindingScopeType;
use sea_orm::entity::prelude::DateTime;

use crate::{
  error::{AppError, ToAppError},
  rbac::permissions::{codes, roles},
  repository::{
    PermissionCreateData, Repository, RoleCreateData, RolePermissionCreateData,
    UserRoleBindingCreateData,
  },
};

const SYSTEM_PERMISSIONS: [(&str, &str); 5] = [
  (codes::SITE_MANAGE, "Manage sites"),
  (codes::COMMENT_MODERATE, "Moderate comments"),
  (codes::COMMENT_DELETE_ANY, "Delete any comment"),
  (codes::OAUTH_PROVIDER_MANAGE, "Manage OAuth providers"),
  (
    codes::MODERATION_PROVIDER_MANAGE,
    "Manage moderation providers",
  ),
];

const SYSTEM_ROLES: [(&str, Option<&str>); 3] = [
  (roles::SUPER_ADMIN, Some("System super administrator")),
  (roles::SITE_ADMIN, Some("Site administrator")),
  (roles::MODERATOR, Some("Comment moderator")),
];

const SUPER_ADMIN_PERMISSION_CODES: [&str; 5] = [
  codes::SITE_MANAGE,
  codes::COMMENT_MODERATE,
  codes::COMMENT_DELETE_ANY,
  codes::OAUTH_PROVIDER_MANAGE,
  codes::MODERATION_PROVIDER_MANAGE,
];

const SITE_ADMIN_PERMISSION_CODES: [&str; 3] = [
  codes::SITE_MANAGE,
  codes::OAUTH_PROVIDER_MANAGE,
  codes::MODERATION_PROVIDER_MANAGE,
];

const MODERATOR_PERMISSION_CODES: [&str; 2] = [codes::COMMENT_MODERATE, codes::COMMENT_DELETE_ANY];

pub async fn bootstrap_rbac(repo: &Repository) -> Result<(), AppError> {
  let datetime = utc_now().naive_utc();

  for (name, description) in SYSTEM_PERMISSIONS {
    if repo
      .permission()
      .find_by_name(name)
      .await
      .with_op("find permission by name")?
      .is_none()
    {
      repo
        .permission()
        .create(PermissionCreateData {
          name: name.to_string(),
          description: description.to_string().into(),
          datetime,
        })
        .await
        .with_op("create permission")?;
    }
  }

  for (name, description) in SYSTEM_ROLES {
    if repo
      .role()
      .find_by_name(name)
      .await
      .with_op("find role by name")?
      .is_none()
    {
      repo
        .role()
        .create(RoleCreateData {
          name: name.to_string(),
          description: description.map(str::to_string),
          datetime,
        })
        .await
        .with_op("create role")?;
    }
  }

  ensure_role_permissions(repo, roles::SUPER_ADMIN, &SUPER_ADMIN_PERMISSION_CODES, datetime).await?;
  ensure_role_permissions(repo, roles::SITE_ADMIN, &SITE_ADMIN_PERMISSION_CODES, datetime).await?;
  ensure_role_permissions(repo, roles::MODERATOR, &MODERATOR_PERMISSION_CODES, datetime).await?;

  Ok(())
}

pub async fn ensure_super_admin_binding(repo: &Repository, user_id: i64) -> Result<(), AppError> {
  let datetime = utc_now().naive_utc();
  let role = repo
    .role()
    .find_by_name(roles::SUPER_ADMIN)
    .await
    .with_op("find super admin role by name")?
    .ok_or(AppError::Internal {
      msg: "super_admin role not initialized".to_string(),
      source: None,
    })?;

  let existing = repo
    .user_role_binding()
    .find_all_by_user_and_scope(user_id, UserRoleBindingScopeType::Global, None)
    .await
    .with_op("find user global role bindings")?;

  if existing.iter().any(|binding| binding.role_id == role.id) {
    return Ok(());
  }

  repo
    .user_role_binding()
    .create(UserRoleBindingCreateData {
      user_id,
      role_id: role.id,
      scope_type: UserRoleBindingScopeType::Global,
      scope_id: None,
      datetime,
    })
    .await
    .with_op("create super admin user role binding")?;

  Ok(())
}

async fn ensure_role_permissions(
  repo: &Repository,
  role_name: &str,
  permission_codes: &[&str],
  datetime: DateTime,
) -> Result<(), AppError> {
  let role = repo
    .role()
    .find_by_name(role_name)
    .await
    .with_op("find role by name")?
    .ok_or(AppError::Internal {
      msg: format!("role {} not initialized", role_name),
      source: None,
    })?;

  for permission_code in permission_codes {
    let permission = repo
      .permission()
      .find_by_name(permission_code)
      .await
      .with_op("find permission by name")?
      .ok_or(AppError::Internal {
        msg: format!("permission {} not initialized", permission_code),
        source: None,
      })?;

    if repo
      .role_permission()
      .find_by_role_and_permission(role.id, permission.id)
      .await
      .with_op("find role permission")?
      .is_none()
    {
      repo
        .role_permission()
        .create(RolePermissionCreateData {
          role_id: role.id,
          permission_id: permission.id,
          datetime,
        })
        .await
        .with_op("create role permission")?;
    }
  }

  Ok(())
}
