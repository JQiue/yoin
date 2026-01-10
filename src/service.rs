use helpers::{
  hash::argon2,
  jwt,
  time::utc_now,
  uuid::{Alphabet, nanoid},
};
use migration::enums::UserRole;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection};

use crate::{
  entity::{prelude::Users, users},
  error::{AppError, ToAppError},
  handler::RegisterResponse,
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
