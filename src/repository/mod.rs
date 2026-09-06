pub mod comment;
pub mod moderation_provider;
pub mod oauth_provider;
pub mod permission;
pub mod reaction;
pub mod role;
pub mod role_permission;
pub mod site;
pub mod user;
pub mod user_identity;
pub mod user_role_binding;

pub use comment::{CommentCreateData, CommentListVisibility, CommentRepository};
pub use moderation_provider::{
  ModerationProviderCreateData, ModerationProviderRepository, ModerationProviderUpdateData,
};
pub use oauth_provider::{
  OauthProviderCreateData, OauthProviderRepository, OauthProviderUpdateData,
};
pub use permission::{PermissionCreateData, PermissionRepository, PermissionUpdateData};
pub use reaction::{ReactionCreateData, ReactionRepository};
pub use role::{RoleCreateData, RoleRepository, RoleUpdateData};
pub use role_permission::{RolePermissionCreateData, RolePermissionRepository};
pub use site::{SiteCreateData, SiteRepository, SiteUpdateData};
pub use user::{UserCreateData, UserRepository, UserUpdateData};
pub use user_identity::{UserIdentityCreateData, UserIdentityRepository, UserIdentityUpdateData};
pub use user_role_binding::{
  UserRoleBindingCreateData, UserRoleBindingRepository, UserRoleBindingUpdateData,
};

#[rustfmt::skip]
use sea_orm::DatabaseConnection;

pub struct Repository {
  pub conn: &'static DatabaseConnection,
}

impl Repository {
  pub fn new(conn: &'static DatabaseConnection) -> Self {
    Self { conn }
  }

  pub fn user(&self) -> UserRepository {
    UserRepository { conn: self.conn }
  }

  pub fn site(&self) -> SiteRepository {
    SiteRepository { conn: self.conn }
  }

  pub fn comment(&self) -> CommentRepository {
    CommentRepository { conn: self.conn }
  }

  pub fn reaction(&self) -> ReactionRepository {
    ReactionRepository { conn: self.conn }
  }

  pub fn oauth_provider(&self) -> OauthProviderRepository {
    OauthProviderRepository { conn: self.conn }
  }

  pub fn moderation_provider(&self) -> ModerationProviderRepository {
    ModerationProviderRepository { conn: self.conn }
  }

  pub fn role(&self) -> RoleRepository {
    RoleRepository { conn: self.conn }
  }

  pub fn permission(&self) -> PermissionRepository {
    PermissionRepository { conn: self.conn }
  }

  pub fn role_permission(&self) -> RolePermissionRepository {
    RolePermissionRepository { conn: self.conn }
  }

  pub fn user_identity(&self) -> UserIdentityRepository {
    UserIdentityRepository { conn: self.conn }
  }

  pub fn user_role_binding(&self) -> UserRoleBindingRepository {
    UserRoleBindingRepository { conn: self.conn }
  }
}
