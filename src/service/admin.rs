use std::collections::{BTreeMap, BTreeSet, HashMap};

use helpers::time::utc_now;
use migration::enums::{CommentStatus, UserRoleBindingScopeType};

use super::AppService;
use crate::{
  error::{AppError, ToAppError},
  handler::admin::{
    AdminCapabilitiesView, CommentViewForAdmin, CreateUserRoleBindingPayload, ListQueryString,
    PageResponse, PermissionView, RoleView, UserBindingSummaryView, UserIdentityView,
    UserRoleBindingView, UserView,
  },
  rbac::permissions::{codes::SITE_MANAGE, roles::SUPER_ADMIN},
  repository::{RolePermissionCreateData, UserRoleBindingCreateData},
};

impl AppService {
  /// Get an admin user's capabilities.
  ///
  /// This derives both global permissions and per-site permissions by joining
  /// role bindings with role->permission mappings.
  pub async fn get_admin_capabilities(
    &self,
    user_id: i64,
  ) -> Result<AdminCapabilitiesView, AppError> {
    let bindings = self
      .repo
      .user_role_binding()
      .find_all_by_user_id(user_id)
      .await
      .with_op("find user role bindings")?;

    let permissions = self
      .repo
      .permission()
      .find_all()
      .await
      .with_op("find all permissions")?;
    let permission_map: HashMap<i64, String> =
      permissions.into_iter().map(|p| (p.id, p.name)).collect();

    let mut global_permissions = BTreeSet::new();
    let mut site_permissions: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();

    for binding in bindings {
      let role_permissions = self
        .repo
        .role_permission()
        .find_all_by_role_id(binding.role_id)
        .await
        .with_op("find role permissions by role id")?;

      for role_permission in role_permissions {
        let Some(permission_name) = permission_map.get(&role_permission.permission_id) else {
          continue;
        };

        match binding.scope_type {
          UserRoleBindingScopeType::Global => {
            global_permissions.insert(permission_name.clone());
          }
          UserRoleBindingScopeType::Site => {
            if let Some(scope_id) = binding.scope_id.as_ref() {
              site_permissions
                .entry(scope_id.clone())
                .or_default()
                .insert(permission_name.clone());
            }
          }
        }
      }
    }

    Ok(AdminCapabilitiesView {
      global_permissions: global_permissions.into_iter().collect(),
      site_permissions: site_permissions
        .into_iter()
        .map(|(site_id, permissions)| (site_id, permissions.into_iter().collect()))
        .collect(),
    })
  }

  /// List roles available to an admin user.
  ///
  /// Requires the admin to have the `SITE_MANAGE` permission globally.
  pub async fn list_roles_for_admin(&self, user_id: i64) -> Result<Vec<RoleView>, AppError> {
    self.require_global_permission(user_id, SITE_MANAGE).await?;

    let roles = self
      .repo
      .role()
      .find_all()
      .await
      .with_op("find all roles")?;
    let permissions = self
      .repo
      .permission()
      .find_all()
      .await
      .with_op("find all permissions")?;
    let permission_map: HashMap<i64, String> =
      permissions.into_iter().map(|p| (p.id, p.name)).collect();

    let mut items = Vec::with_capacity(roles.len());
    for role in roles {
      let role_permissions = self
        .repo
        .role_permission()
        .find_all_by_role_id(role.id)
        .await
        .with_op("find role permissions by role id")?;
      let permission_names = role_permissions
        .into_iter()
        .filter_map(|rp| permission_map.get(&rp.permission_id).cloned())
        .collect();

      items.push(RoleView {
        id: role.id,
        name: role.name,
        description: role.description,
        permission_names,
      });
    }

    Ok(items)
  }

  /// List all permissions (with descriptions) that an admin can view.
  ///
  /// Requires the admin to have the `SITE_MANAGE` permission globally.
  pub async fn list_permissions_for_admin(
    &self,
    user_id: i64,
  ) -> Result<Vec<PermissionView>, AppError> {
    self.require_global_permission(user_id, SITE_MANAGE).await?;

    let permissions = self
      .repo
      .permission()
      .find_all()
      .await
      .with_op("find all permissions")?;

    Ok(
      permissions
        .into_iter()
        .map(|permission| PermissionView {
          id: permission.id,
          name: permission.name,
          description: permission.description,
        })
        .collect(),
    )
  }

  /// List user-role bindings for the admin, including resolved role names.
  ///
  /// Requires the admin to have the `SITE_MANAGE` permission globally.
  pub async fn list_user_role_bindings_for_admin(
    &self,
    user_id: i64,
  ) -> Result<Vec<UserRoleBindingView>, AppError> {
    self.require_global_permission(user_id, SITE_MANAGE).await?;

    let bindings = self
      .repo
      .user_role_binding()
      .find_all()
      .await
      .with_op("find all user role bindings")?;
    let roles = self
      .repo
      .role()
      .find_all()
      .await
      .with_op("find all roles")?;
    let role_map: HashMap<i64, String> =
      roles.into_iter().map(|role| (role.id, role.name)).collect();

    Ok(
      bindings
        .into_iter()
        .map(|binding| UserRoleBindingView {
          id: binding.id,
          user_id: binding.user_id,
          role_id: binding.role_id,
          role_name: role_map.get(&binding.role_id).cloned(),
          scope_type: match binding.scope_type {
            UserRoleBindingScopeType::Global => "global".to_string(),
            UserRoleBindingScopeType::Site => "site".to_string(),
          },
          scope_id: binding.scope_id,
          created_at: binding.created_at.and_utc().to_rfc3339(),
          updated_at: binding.updated_at.and_utc().to_rfc3339(),
        })
        .collect(),
    )
  }

  fn binding_scope_type(value: &str) -> Result<UserRoleBindingScopeType, AppError> {
    match value {
      "global" => Ok(UserRoleBindingScopeType::Global),
      "site" => Ok(UserRoleBindingScopeType::Site),
      _ => Err(AppError::bad_request("invalid scope_type".to_string())),
    }
  }

  fn binding_scope_label(scope_type: UserRoleBindingScopeType) -> String {
    match scope_type {
      UserRoleBindingScopeType::Global => "global".to_string(),
      UserRoleBindingScopeType::Site => "site".to_string(),
    }
  }

  async fn role_view_by_id(&self, role_id: i64) -> Result<RoleView, AppError> {
    let role = self
      .repo
      .role()
      .find_by_id(role_id)
      .await
      .with_op("find role by id")?
      .ok_or_else(|| AppError::bad_request("role not found".to_string()))?;
    let permissions = self
      .repo
      .permission()
      .find_all()
      .await
      .with_op("find all permissions")?;
    let permission_map: HashMap<i64, String> =
      permissions.into_iter().map(|p| (p.id, p.name)).collect();
    let role_permissions = self
      .repo
      .role_permission()
      .find_all_by_role_id(role.id)
      .await
      .with_op("find role permissions by role id")?;
    let permission_names = role_permissions
      .into_iter()
      .filter_map(|rp| permission_map.get(&rp.permission_id).cloned())
      .collect();
    Ok(RoleView {
      id: role.id,
      name: role.name,
      description: role.description,
      permission_names,
    })
  }

  async fn user_role_binding_view(
    &self,
    binding: crate::entity::user_role_bindings::Model,
  ) -> Result<UserRoleBindingView, AppError> {
    let role = self
      .repo
      .role()
      .find_by_id(binding.role_id)
      .await
      .with_op("find role by id")?;
    Ok(UserRoleBindingView {
      id: binding.id,
      user_id: binding.user_id,
      role_id: binding.role_id,
      role_name: role.map(|role| role.name),
      scope_type: Self::binding_scope_label(binding.scope_type),
      scope_id: binding.scope_id,
      created_at: binding.created_at.and_utc().to_rfc3339(),
      updated_at: binding.updated_at.and_utc().to_rfc3339(),
    })
  }

  async fn count_global_super_admin_bindings(&self) -> Result<usize, AppError> {
    let role = self
      .repo
      .role()
      .find_by_name(SUPER_ADMIN)
      .await
      .with_op("find super admin role")?
      .ok_or_else(|| AppError::Internal {
        msg: "super_admin role not initialized".to_string(),
        source: None,
      })?;
    let bindings = self
      .repo
      .user_role_binding()
      .find_all_by_role_and_scope(role.id, UserRoleBindingScopeType::Global, None)
      .await
      .with_op("find global super admin bindings")?;
    Ok(bindings.len())
  }

  /// List users for the admin panel, including identities and role bindings.
  ///
  /// Requires the admin to have the `SITE_MANAGE` permission globally.
  pub async fn list_users_for_admin(&self, user_id: i64) -> Result<Vec<UserView>, AppError> {
    self.require_global_permission(user_id, SITE_MANAGE).await?;

    let users = self
      .repo
      .user()
      .find_all()
      .await
      .with_op("find all users")?;
    let roles = self
      .repo
      .role()
      .find_all()
      .await
      .with_op("find all roles")?;
    let role_map: HashMap<i64, String> =
      roles.into_iter().map(|role| (role.id, role.name)).collect();

    let mut items = Vec::with_capacity(users.len());
    for user in users {
      let identities = self
        .repo
        .user_identity()
        .find_all_by_user_id(user.id)
        .await
        .with_op("find user identities")?;
      let bindings = self
        .repo
        .user_role_binding()
        .find_all_by_user_id(user.id)
        .await
        .with_op("find user role bindings")?;
      items.push(UserView {
        id: user.id,
        nickname: user.nickname,
        email: user.email,
        website: user.website,
        avatar: user.avatar,
        created_at: user.created_at.and_utc().to_rfc3339(),
        identities: identities
          .into_iter()
          .map(|identity| UserIdentityView {
            id: identity.id,
            provider: identity.provider,
            provider_user_id: identity.provider_user_id,
            email: identity.email,
          })
          .collect(),
        role_bindings: bindings
          .into_iter()
          .map(|binding| UserBindingSummaryView {
            id: binding.id,
            role_id: binding.role_id,
            role_name: role_map.get(&binding.role_id).cloned(),
            scope_type: Self::binding_scope_label(binding.scope_type),
            scope_id: binding.scope_id,
          })
          .collect(),
      });
    }

    Ok(items)
  }

  /// Replace the permission set of a role.
  ///
  /// Requires the admin to have the `SITE_MANAGE` permission globally.
  pub async fn replace_role_permissions_for_admin(
    &self,
    user_id: i64,
    role_id: i64,
    permission_names: Vec<String>,
  ) -> Result<RoleView, AppError> {
    self.require_global_permission(user_id, SITE_MANAGE).await?;

    let role = self
      .repo
      .role()
      .find_by_id(role_id)
      .await
      .with_op("find role by id")?
      .ok_or_else(|| AppError::bad_request("role not found".to_string()))?;

    let mut permission_ids = Vec::with_capacity(permission_names.len());
    let mut seen = BTreeSet::new();
    for name in permission_names {
      if !seen.insert(name.clone()) {
        continue;
      }
      let permission = self
        .repo
        .permission()
        .find_by_name(&name)
        .await
        .with_op("find permission by name")?
        .ok_or_else(|| AppError::bad_request(format!("unknown permission: {name}")))?;
      permission_ids.push(permission.id);
    }

    if role.name == SUPER_ADMIN {
      let site_manage = self
        .repo
        .permission()
        .find_by_name(SITE_MANAGE)
        .await
        .with_op("find site.manage permission")?
        .ok_or_else(|| AppError::Internal {
          msg: "site.manage permission not initialized".to_string(),
          source: None,
        })?;
      if !permission_ids.contains(&site_manage.id) {
        return Err(AppError::bad_request(
          "super_admin must keep site.manage".to_string(),
        ));
      }
    }

    self
      .repo
      .role_permission()
      .delete_by_role_id(role.id)
      .await
      .with_op("delete role permissions")?;

    let datetime = utc_now().naive_utc();
    for permission_id in permission_ids {
      self
        .repo
        .role_permission()
        .create(RolePermissionCreateData {
          role_id: role.id,
          permission_id,
          datetime,
        })
        .await
        .with_op("create role permission")?;
    }

    self.role_view_by_id(role.id).await
  }

  /// Create a user-role binding.
  ///
  /// Requires the admin to have the `SITE_MANAGE` permission globally.
  pub async fn create_user_role_binding_for_admin(
    &self,
    user_id: i64,
    payload: CreateUserRoleBindingPayload,
  ) -> Result<UserRoleBindingView, AppError> {
    self.require_global_permission(user_id, SITE_MANAGE).await?;

    let scope_type = Self::binding_scope_type(&payload.scope_type)?;
    let scope_id = match scope_type {
      UserRoleBindingScopeType::Global => {
        if payload.scope_id.is_some() {
          return Err(AppError::bad_request(
            "global bindings must not have scope_id".to_string(),
          ));
        }
        None
      }
      UserRoleBindingScopeType::Site => {
        let Some(scope_id) = payload.scope_id.filter(|value| !value.is_empty()) else {
          return Err(AppError::bad_request(
            "site bindings require scope_id".to_string(),
          ));
        };
        let site_id = scope_id
          .parse::<i64>()
          .map_err(|_| AppError::bad_request("invalid site scope_id".to_string()))?;
        self
          .repo
          .site()
          .find_by_id(site_id)
          .await
          .with_op("find site by id")?
          .ok_or_else(|| AppError::site_not_found("Site not found".to_string()))?;
        Some(scope_id)
      }
    };

    self
      .repo
      .user()
      .find_by_id(payload.user_id)
      .await
      .with_op("find user by id")?
      .ok_or_else(|| AppError::user_not_found("User not found".to_string()))?;
    self
      .repo
      .role()
      .find_by_id(payload.role_id)
      .await
      .with_op("find role by id")?
      .ok_or_else(|| AppError::bad_request("role not found".to_string()))?;

    let existing = self
      .repo
      .user_role_binding()
      .find_all_by_user_and_scope(payload.user_id, scope_type.clone(), scope_id.as_deref())
      .await
      .with_op("find user role bindings by scope")?;
    if existing
      .iter()
      .any(|binding| binding.role_id == payload.role_id)
    {
      return Err(AppError::bad_request(
        "role binding already exists".to_string(),
      ));
    }

    let binding = self
      .repo
      .user_role_binding()
      .create(UserRoleBindingCreateData {
        user_id: payload.user_id,
        role_id: payload.role_id,
        scope_type,
        scope_id,
        datetime: utc_now().naive_utc(),
      })
      .await
      .with_op("create user role binding")?;

    self.user_role_binding_view(binding).await
  }

  /// Delete a user-role binding.
  ///
  /// Requires the admin to have the `SITE_MANAGE` permission globally.
  /// The last global `super_admin` binding cannot be removed.
  pub async fn delete_user_role_binding_for_admin(
    &self,
    user_id: i64,
    binding_id: i64,
  ) -> Result<(), AppError> {
    self.require_global_permission(user_id, SITE_MANAGE).await?;

    let binding = self
      .repo
      .user_role_binding()
      .find_by_id(binding_id)
      .await
      .with_op("find user role binding by id")?
      .ok_or_else(|| AppError::bad_request("role binding not found".to_string()))?;

    if binding.scope_type == UserRoleBindingScopeType::Global && binding.scope_id.is_none() {
      let role = self
        .repo
        .role()
        .find_by_id(binding.role_id)
        .await
        .with_op("find role by id")?;
      if role.is_some_and(|role| role.name == SUPER_ADMIN)
        && self.count_global_super_admin_bindings().await? <= 1
      {
        return Err(AppError::bad_request(
          "cannot remove the last global super_admin".to_string(),
        ));
      }
    }

    self
      .repo
      .user_role_binding()
      .delete_by_id(binding_id)
      .await
      .with_op("delete user role binding")?;
    Ok(())
  }

  /// List comments for an admin panel, with pagination and status filtering.
  pub async fn list_comments_for_admin(
    &self,
    _user_id: i64,
    qs: ListQueryString,
  ) -> Result<PageResponse<CommentViewForAdmin>, AppError> {
    let (replies, total, total_pages) = self
      .repo
      .comment()
      .find_all_paged(qs.page_size, qs.page_offset, &qs.sort, qs.status)
      .await
      .with_op("query replies")?;
    let items = replies
      .into_iter()
      .map(CommentViewForAdmin::from_model)
      .collect();
    Ok(PageResponse {
      items,
      page_size: qs.page_size,
      page_offset: qs.page_offset,
      total_pages,
      total,
    })
  }

  /// Update the status of a comment from the admin side.
  pub async fn update_comment_status_for_admin(
    &self,
    _user_id: i64,
    id: i64,
    status: CommentStatus,
  ) -> Result<(), AppError> {
    self
      .repo
      .comment()
      .update_status(id, status)
      .await
      .with_op("update comment status")?;
    Ok(())
  }

  /// Delete (soft-delete) a comment as an admin.
  ///
  /// If the target comment is a root comment (`parent_id` is `None`), the
  /// whole thread is deleted; otherwise, only the comment is deleted.
  pub async fn delete_comment_for_admin(&self, _user_id: i64, id: i64) -> Result<(), AppError> {
    let comment = self
      .repo
      .comment()
      .find_by_id(id)
      .await
      .with_op("find comment by id")?
      .ok_or(AppError::comment_not_found("Comment not found".to_string()))?;

    if comment.parent_id.is_none() {
      self
        .repo
        .comment()
        .soft_delete_thread(comment.id)
        .await
        .with_op("delete comment thread")?;
    } else {
      self
        .repo
        .comment()
        .soft_delete(comment)
        .await
        .with_op("delete comment")?;
    }

    Ok(())
  }
}
