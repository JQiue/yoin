use helpers::{
  hash::{argon2, verify_argon2},
  jwt,
  time::utc_now,
  uuid::{Alphabet, nanoid},
};

use super::AppService;
use crate::{
  error::{AppError, ToAppError},
  handler::{
    auth::{LoginPayload, UserProfile, UserWithToken},
    user::UpdateProfilePayload,
  },
  helper::generate_avatar,
  rbac::bootstrap::ensure_super_admin_binding,
  repository::{UserCreateData, UserUpdateData},
};

impl AppService {
  pub async fn create_user(
    &self,
    nickname: String,
    website: String,
    email: String,
    password: String,
    jwt_key: &str,
  ) -> Result<UserWithToken, AppError> {
    if self
      .repo
      .user()
      .exists_by_email(&email)
      .await
      .with_op("has_user_by_email")?
    {
      return Err(AppError::user_already_exists(
        "User already exists".to_string(),
      ));
    }

    let datetime = utc_now().naive_utc();
    let is_first_user = self
      .repo
      .user()
      .exists_any()
      .await
      .with_op("is first user")?;

    if is_first_user {
      self
        .repo
        .site()
        .create_default(datetime)
        .await
        .with_op("create default site")?;
    }

    let password = argon2(&password, &nanoid(&Alphabet::DEFAULT, 8)).with_op("hash_password")?;
    let new_user = self
      .repo
      .user()
      .create(UserCreateData {
        nickname,
        password,
        email: email.clone(),
        website,
        avatar: generate_avatar(&email),
        datetime,
      })
      .await
      .with_op("insert_user")?;

    if is_first_user {
      ensure_super_admin_binding(&self.repo, new_user.id).await?;
    }

    let token = jwt::sign(new_user.id, jwt_key, 30 * 24 * 60 * 60).with_op("sign jwt token")?;

    Ok(UserWithToken {
      token,
      user: UserProfile {
        avatar: new_user.avatar,
        nickname: new_user.nickname,
        website: new_user.website,
        email,
      },
    })
  }

  pub async fn login_user(
    &self,
    payload: LoginPayload,
    jwt_key: &str,
  ) -> Result<UserWithToken, AppError> {
    let user = self
      .repo
      .user()
      .find_by_email(&payload.email)
      .await
      .with_op("get_user_by_email")?
      .ok_or(AppError::user_not_found("User not found".to_string()))?;

    if !verify_argon2(&user.password, &payload.password).with_op("verify_argon2")? {
      return Err(AppError::invalid_credentials(
        "Invalid email or password".to_string(),
      ));
    }

    let token = jwt::sign(user.id, jwt_key, 30 * 24 * 60 * 60).with_op("sign jwt token")?;

    Ok(UserWithToken {
      user: UserProfile {
        avatar: user.avatar,
        nickname: user.nickname,
        website: user.website,
        email: user.email,
      },
      token,
    })
  }

  pub async fn fetch_profile(&self, user_id: i64) -> Result<UserProfile, AppError> {
    let user = self
      .repo
      .user()
      .find_by_id(user_id)
      .await
      .with_op("get_user_by_id")?
      .ok_or(AppError::user_not_found("User not found".to_string()))?;
    Ok(UserProfile {
      avatar: user.avatar,
      nickname: user.nickname,
      website: user.website,
      email: user.email,
    })
  }

  pub async fn update_user_profile(
    &self,
    user_id: i64,
    payload: UpdateProfilePayload,
  ) -> Result<UserProfile, AppError> {
    let user = self
      .repo
      .user()
      .find_by_id(user_id)
      .await
      .with_op("get_user_by_id")?
      .ok_or(AppError::user_not_found("User not found".to_string()))?;

    let updated_user = self
      .repo
      .user()
      .update(UserUpdateData {
        id: user.id,
        nickname: payload.nickname,
        website: payload.website,
        avatar: payload.avatar,
        datetime: utc_now().naive_utc(),
        ..Default::default()
      })
      .await
      .with_op("update_user")?;
    Ok(UserProfile {
      avatar: updated_user.avatar,
      nickname: updated_user.nickname,
      website: updated_user.website,
      email: updated_user.email,
    })
  }
}
