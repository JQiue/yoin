use helpers::time::utc_now;
use migration::enums::UserRoleBindingScopeType;
use sea_orm::entity::prelude::DateTime;

use crate::{
  constants::rbac::{
    MODERATOR_PERMISSION_CODES, SITE_ADMIN_PERMISSION_CODES, SUPER_ADMIN_PERMISSION_CODES,
    SYSTEM_PERMISSIONS, SYSTEM_ROLES, roles,
  },
  error::{AppError, ToAppError},
  repository::{
    PermissionCreateData, Repository, RoleCreateData, RolePermissionCreateData,
    UserRoleBindingCreateData,
  },
};

/// Bootstrap RBAC
///
#[doc = "docs"]
#[doc = include_str!("../docs/bootstrap_rbac.md")]
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

  ensure_role_permissions(
    repo,
    roles::SUPER_ADMIN,
    &SUPER_ADMIN_PERMISSION_CODES,
    datetime,
  )
  .await?;
  ensure_role_permissions(
    repo,
    roles::SITE_ADMIN,
    &SITE_ADMIN_PERMISSION_CODES,
    datetime,
  )
  .await?;
  ensure_role_permissions(
    repo,
    roles::MODERATOR,
    &MODERATOR_PERMISSION_CODES,
    datetime,
  )
  .await?;

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
