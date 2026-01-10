use helpers::{
  hash::{argon2, verify_argon2},
  jwt,
  time::utc_now,
  uuid::{Alphabet, nanoid},
};
use migration::enums::UserRole;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection, IntoActiveModel};

use crate::{
  entity::{prelude::Users, users},
  error::{AppError, ToAppError},
  handler::{LoginResponse, RegisterResponse, UpdateProfileResponse},
  helper::generate_avatar,
  repo::UserRepo,
};

pub async fn create_user(
  nickname: String,
  url: String,
  email: String,
  password: String,
  conn: &DatabaseConnection,
  jwt_key: &str,
) -> Result<RegisterResponse, AppError> {
  if Users::has_user_by_email(&email, conn)
    .await
    .with_op("has_user_by_email")?
  {
    return Err(AppError::user_already_exists(
      "User already exists".to_string(),
    ));
  }

  let role = if Users::is_first_user(conn).await.with_op("is_first_user")? {
    UserRole::Admin
  } else {
    UserRole::Normal
  };
  let hashed = argon2(&password, &nanoid(&Alphabet::DEFAULT, 8)).with_op("hash_password")?;
  let datetime = utc_now().naive_utc();
  let insert_user = users::ActiveModel {
    nickname: Set(nickname),
    password: Set(hashed),
    email: Set(email.clone()),
    avatar: Set(generate_avatar(&email)),
    role: Set(role),
    url: Set(url),
    created_at: Set(datetime),
    updated_at: Set(datetime),
    ..Default::default()
  };

  let new_user = insert_user.insert(conn).await.with_op("insert_user")?;
  let token = jwt::sign(new_user.id, jwt_key, 30 * 24 * 60 * 60).with_op("sign jwt token")?;

  Ok(RegisterResponse {
    avatar: new_user.avatar,
    nickname: new_user.nickname,
    url: new_user.url,
    token,
  })
}

pub async fn get_token(
  email: String,
  password: String,
  conn: &DatabaseConnection,
  jwt_key: &str,
) -> Result<LoginResponse, AppError> {
  let user = Users::get_user_by_email(&email, conn)
    .await
    .with_op("get_user_by_email")?
    .ok_or(AppError::user_not_found("User not found".to_string()))?;

  if verify_argon2(&user.password, &password).with_op("verify_argon2")? {
    return Err(AppError::invalid_credentials(
      "Invalid email or password".to_string(),
    ));
  }

  let token = jwt::sign(user.id, jwt_key, 30 * 24 * 60 * 60).with_op("sign jwt token")?;

  Ok(LoginResponse {
    avatar: user.avatar,
    nickname: user.nickname,
    url: user.url,
    // datetime: user.created_at.and_utc().to_rfc3339(),
    token,
  })
}

pub async fn update_my_profile(
  user_id: i32,
  nickname: Option<String>,
  avatar: Option<String>,
  url: Option<String>,
  conn: &DatabaseConnection,
) -> Result<UpdateProfileResponse, AppError> {
  let user = Users::get_user_by_id(user_id, conn)
    .await
    .with_op("get_user_by_id")?
    .ok_or(AppError::user_not_found("User not found".to_string()))?;
  let mut update_user = user.into_active_model();
  if let Some(nickname) = nickname {
    update_user.nickname = Set(nickname);
  }
  if let Some(avatar) = avatar {
    update_user.avatar = Set(avatar);
  }
  if let Some(url) = url {
    update_user.url = Set(url);
  }
  let updated_user = update_user.update(conn).await.with_op("update_user")?;
  Ok(UpdateProfileResponse {
    avatar: updated_user.avatar,
    nickname: updated_user.nickname,
    url: updated_user.url,
  })
}
