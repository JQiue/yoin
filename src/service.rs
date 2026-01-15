use helpers::{
  hash::{argon2, verify_argon2},
  jwt,
  time::utc_now,
  uuid::{Alphabet, nanoid},
};
use migration::enums::UserRole;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection, IntoActiveModel};

use crate::{
  entity::{
    prelude::{Sites, Users},
    sites::{self, SiteConfig},
    users,
  },
  error::{AppError, ToAppError},
  handler::{
    auth::{LoginPayload, UserProfile, UserWithToken},
    site::{CreateSitePayload, SiteView},
    user::UpdateProfilePayload,
  },
  helper::generate_avatar,
  repo::{SiteRepo, UserRepo},
};

pub async fn create_user(
  nickname: String,
  url: String,
  email: String,
  password: String,
  conn: &DatabaseConnection,
  jwt_key: &str,
) -> Result<UserWithToken, AppError> {
  if Users::exists_by_email(&email, conn)
    .await
    .with_op("has_user_by_email")?
  {
    return Err(AppError::user_already_exists(
      "User already exists".to_string(),
    ));
  }
  let datetime = utc_now().naive_utc();
  let role = if Users::exists_any(conn).await.with_op("is first user")? {
    let new_site = sites::ActiveModel {
      name: Set("Default Site".to_string()),
      url: Set("".to_string()),
      config: Set(SiteConfig::default()),
      created_at: Set(datetime),
      updated_at: Set(datetime),
      ..Default::default()
    };
    new_site.insert(conn).await.with_op("insert_site")?;
    UserRole::Admin
  } else {
    UserRole::Normal
  };
  let hashed = argon2(&password, &nanoid(&Alphabet::DEFAULT, 8)).with_op("hash_password")?;
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

  Ok(UserWithToken {
    token,
    user: UserProfile {
      avatar: new_user.avatar,
      nickname: new_user.nickname,
      url: new_user.url,
    },
  })
}

pub async fn login_user(
  payload: LoginPayload,
  conn: &DatabaseConnection,
  jwt_key: &str,
) -> Result<UserWithToken, AppError> {
  let user = Users::find_by_email(&payload.email, conn)
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
      url: user.url,
    },
    // datetime: user.created_at.and_utc().to_rfc3339(),
    token,
  })
}

pub async fn fetch_profile(
  user_id: i64,
  conn: &DatabaseConnection,
) -> Result<UserProfile, AppError> {
  let user = Users::find_by_id(user_id, conn)
    .await
    .with_op("get_user_by_id")?
    .ok_or(AppError::user_not_found("User not found".to_string()))?;
  Ok(UserProfile {
    avatar: user.avatar,
    nickname: user.nickname,
    url: user.url,
  })
}

pub async fn update_user_profile(
  user_id: i64,
  payload: UpdateProfilePayload,
  conn: &DatabaseConnection,
) -> Result<UserProfile, AppError> {
  let user = Users::find_by_id(user_id, conn)
    .await
    .with_op("get_user_by_id")?
    .ok_or(AppError::user_not_found("User not found".to_string()))?;
  let mut update_user = user.into_active_model();
  if let Some(nickname) = payload.nickname {
    update_user.nickname = Set(nickname);
  }
  if let Some(avatar) = payload.avatar {
    update_user.avatar = Set(avatar);
  }
  if let Some(url) = payload.url {
    update_user.url = Set(url);
  }
  let updated_user = update_user.update(conn).await.with_op("update_user")?;
  Ok(UserProfile {
    avatar: updated_user.avatar,
    nickname: updated_user.nickname,
    url: updated_user.url,
  })
}

pub async fn list_sites(
  user_id: i64,
  conn: &DatabaseConnection,
) -> Result<Vec<SiteView>, AppError> {
  let user = Users::find_by_id(user_id, conn)
    .await
    .with_op("find user by id")?
    .ok_or(AppError::user_not_found("User not found".to_string()))?;

  if user.role != UserRole::Admin {
    return Err(AppError::forbidden(
      "You are not admin".to_string() + &format!("{} {:?}", user.id, user.role),
    ));
  }

  let sites = Sites::find_all(conn)
    .await
    .with_op("find all sites")?
    .iter()
    .map(|site| SiteView {
      id: site.id,
      name: site.name.clone(),
      config: site.config.clone(),
      url: site.url.clone(),
    })
    .collect();
  Ok(sites)
}

pub async fn create_site(
  user_id: i64,
  payload: CreateSitePayload,
  conn: &DatabaseConnection,
) -> Result<SiteView, AppError> {
  let user = Users::find_by_id(user_id, conn)
    .await
    .with_op("find user by id")?
    .ok_or(AppError::user_not_found("User not found".to_string()))?;

  if user.role != UserRole::Admin {
    return Err(AppError::forbidden(
      "You are not admin 1234".to_string() + &user.id.to_string(),
    ));
  }
  let datetime = utc_now().naive_utc();
  let new_site = sites::ActiveModel {
    name: Set(payload.name),
    url: Set(payload.url),
    config: Set(SiteConfig::default()),
    created_at: Set(datetime),
    updated_at: Set(datetime),
    ..Default::default()
  };

  let site = new_site.insert(conn).await.with_op("insert_site")?;
  Ok(SiteView {
    id: site.id,
    name: site.name,
    config: site.config,
    url: site.url,
  })
}
