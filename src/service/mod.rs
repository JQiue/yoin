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
}
