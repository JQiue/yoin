//! Application service layer.
//!
//! This module aggregates the various domain services (admin, comment,
//! moderation, oauth, etc.) that implement business logic on top of the
//! repository layer. Each submodule provides a set of async operations used by
//! the HTTP handlers.

pub mod admin;
mod comment;
mod moderation;
mod oauth;
mod reaction;
mod site;
mod subscription;
mod user;

use migration::enums::UserRoleBindingScopeType;

use crate::{
  error::{AppError, ToAppError},
  repository::Repository,
};

pub struct AppService {
  pub repo: Repository,
}

impl AppService {
  /// Check whether `user_id` has `permission_name` for the given scope.
  ///
  /// Returns `Ok(true)` if any role binding for the user on the provided scope
  /// maps to the permission through `role_permission`; otherwise returns
  /// `Ok(false)`.
  async fn has_permission(
    &self,
    user_id: i64,
    permission_name: &str,
    scope_type: UserRoleBindingScopeType,
    scope_id: Option<&str>,
  ) -> Result<bool, AppError> {
    self
      .repo
      .user()
      .find_by_id(user_id)
      .await
      .with_op("find user by id")?
      .ok_or(AppError::user_not_found("User not found".to_string()))?;

    let permission = self
      .repo
      .permission()
      .find_by_name(permission_name)
      .await
      .with_op("find permission by name")?;

    let Some(permission) = permission else {
      return Ok(false);
    };

    let bindings = self
      .repo
      .user_role_binding()
      .find_all_by_user_and_scope(user_id, scope_type.clone(), scope_id)
      .await
      .with_op("find user role bindings by scope")?;

    if bindings.is_empty() {
      return Ok(false);
    }

    for binding in bindings {
      let rp = self
        .repo
        .role_permission()
        .find_by_role_and_permission(binding.role_id, permission.id)
        .await
        .with_op("find role permission")?;

      if rp.is_some() {
        return Ok(true);
      }
    }

    Ok(false)
  }

  /// Require `permission_name` for the given scope.
  ///
  /// Uses `has_permission` and returns a `forbidden` error when the user does
  /// not have the required permission.
  async fn require_permission(
    &self,
    user_id: i64,
    permission_name: &str,
    scope_type: UserRoleBindingScopeType,
    scope_id: Option<&str>,
  ) -> Result<(), AppError> {
    if !self
      .has_permission(user_id, permission_name, scope_type, scope_id)
      .await?
    {
      return Err(AppError::forbidden(format!(
        "Missing permission: {}",
        permission_name
      )));
    }

    Ok(())
  }

  /// Check global permission (`UserRoleBindingScopeType::Global`).
  pub async fn has_global_permission(
    &self,
    user_id: i64,
    permission_name: &str,
  ) -> Result<bool, AppError> {
    self
      .has_permission(
        user_id,
        permission_name,
        UserRoleBindingScopeType::Global,
        None,
      )
      .await
  }

  /// Require global permission (`UserRoleBindingScopeType::Global`).
  pub async fn require_global_permission(
    &self,
    user_id: i64,
    permission_name: &str,
  ) -> Result<(), AppError> {
    self
      .require_permission(
        user_id,
        permission_name,
        UserRoleBindingScopeType::Global,
        None,
      )
      .await
  }

  /// Check site permission, falling back to the same permission on the global scope.
  pub async fn has_site_permission(
    &self,
    user_id: i64,
    permission_name: &str,
    site_id: i64,
  ) -> Result<bool, AppError> {
    if self.has_global_permission(user_id, permission_name).await? {
      return Ok(true);
    }

    self
      .has_permission(
        user_id,
        permission_name,
        UserRoleBindingScopeType::Site,
        Some(&site_id.to_string()),
      )
      .await
  }

  /// Require a site permission, falling back to the same permission on the global scope.
  pub async fn require_site_permission(
    &self,
    user_id: i64,
    permission_name: &str,
    site_id: i64,
  ) -> Result<(), AppError> {
    if !self
      .has_site_permission(user_id, permission_name, site_id)
      .await?
    {
      return Err(AppError::forbidden(format!(
        "Missing permission: {}",
        permission_name
      )));
    }
    Ok(())
  }
}
