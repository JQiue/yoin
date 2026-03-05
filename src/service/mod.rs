mod comment;
mod oauth;
mod site;
mod user;

use migration::enums::UserRole;

use crate::{
  error::{AppError, ToAppError},
  repository::{Repository, UserRepositoryTrait},
};

pub struct AppService {
  pub repo: Repository,
}

impl AppService {
  async fn require_admin(&self, user_id: i64) -> Result<(), AppError> {
    let user = self
      .repo
      .user()
      .find_by_id(user_id)
      .await
      .with_op("find user by id")?
      .ok_or(AppError::user_not_found("User not found".to_string()))?;

    if user.role != UserRole::Admin {
      return Err(AppError::forbidden(
        "You are not admin".to_string() + &format!("{} {:?}", user.id, user.role),
      ));
    }

    Ok(())
  }
}
