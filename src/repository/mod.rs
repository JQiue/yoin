pub mod comment;
pub mod oauth_provider;
pub mod site;
pub mod user;

pub use comment::*;
pub use oauth_provider::*;
pub use site::*;
pub use user::*;

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

  pub fn oauth_provider(&self) -> OauthProvider {
    OauthProvider { conn: self.conn }
  }
}
