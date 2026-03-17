use std::collections::{BTreeMap, BTreeSet, HashMap};

use migration::enums::{CommentStatus, UserRoleBindingScopeType};

use super::AppService;
use crate::{
  error::{AppError, ToAppError},
  handler::admin::{
    AdminCapabilitiesView, CommentViewForAdmin, ListQueryString, PageResponse, PermissionView,
    RoleView, UserRoleBindingView,
  },
  rbac::permissions::codes::SITE_MANAGE,
};

impl AppService {
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
    let role_map: HashMap<i64, String> = roles.into_iter().map(|role| (role.id, role.name)).collect();

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

  pub async fn list_comments_for_admin(
    &self,
    user_id: i64,
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

  pub async fn update_comment_status_for_admin(
    &self,
    user_id: i64,
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

  pub async fn delete_comment_for_admin(&self, user_id: i64, id: i64) -> Result<(), AppError> {
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
