use helpers::{
  hash::{argon2, verify_argon2},
  jwt,
  time::utc_now,
  uuid::{Alphabet, nanoid},
};
use migration::enums::{CommentStatus, UserRole};
use sea_orm::{
  ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, IntoActiveModel,
};

use crate::{
  entity::{
    comments,
    prelude::{Comments, Sites, Users},
    sites::{self, SiteConfig},
    users,
  },
  error::{AppError, ToAppError},
  handler::{
    auth::{LoginPayload, UserProfile, UserWithToken},
    comment::{CommentView, CreateCommentPayload, ListQueryString, PageResponse},
    site::{CreateSitePayload, SiteView, UpdateSitePayload},
    user::UpdateProfilePayload,
  },
  helper::generate_avatar,
  repo::{CommentRepo, SiteRepo, UserRepo},
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
      role: new_user.role.to_string(),
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
      role: user.role.to_string(),
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
    role: user.role.to_string(),
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
    role: updated_user.role.to_string(),
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
      "You are not admin".to_string() + &user.id.to_string(),
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

pub async fn update_site(
  user_id: i64,
  payload: UpdateSitePayload,
  conn: &DatabaseConnection,
) -> Result<SiteView, AppError> {
  let user = Users::find_by_id(user_id, conn)
    .await
    .with_op("find user by id")?
    .ok_or(AppError::user_not_found("User not found".to_string()))?;

  if user.role != UserRole::Admin {
    return Err(AppError::forbidden(
      "You are not admin".to_string() + &user.id.to_string(),
    ));
  }

  let mut active_site = Sites::find_by_id(payload.id, conn)
    .await
    .with_op("find site by id")?
    .ok_or(AppError::site_not_found("Site not found".to_string()))?
    .into_active_model();

  if let Some(name) = payload.name {
    active_site.name = Set(name);
  }
  if let Some(url) = payload.url {
    active_site.url = Set(url);
  }
  if let Some(config) = payload.config {
    active_site.config = Set(config);
  }

  active_site.updated_at = Set(utc_now().naive_utc());
  let site = active_site.update(conn).await.with_op("update site")?;

  Ok(SiteView {
    id: site.id,
    name: site.name,
    config: site.config,
    url: site.url,
  })
}

pub async fn create_comment(
  user_id: Option<i64>,
  payload: CreateCommentPayload,
  conn: &DatabaseConnection,
) -> Result<CommentView, AppError> {
  let device = "unknown".to_string();
  let location = "unknown".to_string();

  let mut active_comment = comments::ActiveModel {
    site_id: Set(payload.site_id),
    nickname: Set(payload.nickname),
    page_path: Set(payload.page_path),
    link: Set(payload.link),
    content: Set(payload.content),
    created_at: Set(utc_now().naive_utc()),
    updated_at: Set(utc_now().naive_utc()),
    email: Set(payload.email),
    device: Set(device),
    location: Set(location),
    is_sticky: Set(false),
    ..Default::default()
  };

  if let Some(user_id) = user_id {
    active_comment.user_id = Set(Some(user_id));
  }

  if let Some(rid) = payload.rid {
    active_comment.rid = Set(rid);
  }

  let comments::Model {
    id,
    nickname,
    link,
    content,
    created_at,
    up_vote,
    down_vote,
    device,
    location,
    rid,
    is_sticky,
    ..
  } = active_comment
    .insert(conn)
    .await
    .with_op("insert comment")?;

  Ok(CommentView {
    id,
    rid,
    nickname,
    link,
    content,
    up_vote,
    down_vote,
    device,
    location,
    is_sticky,
    created_at: created_at.and_utc().to_rfc3339(),
  })
}

pub async fn list_comments(
  qs: ListQueryString,
  conn: &DatabaseConnection,
) -> Result<PageResponse<CommentView>, AppError> {
  use sea_orm::{EntityTrait, Order, PaginatorTrait, QueryFilter, QueryOrder};

  let (sort_col, sort_ord) = match qs.sort_by.as_str() {
    "created_asc" => (comments::Column::CreatedAt, Order::Asc),
    "created_desc" => (comments::Column::CreatedAt, Order::Desc),
    "up_vote_asc" => (comments::Column::UpVote, Order::Asc),
    "up_vote_desc" => (comments::Column::UpVote, Order::Desc),
    "down_vote_asc" => (comments::Column::DownVote, Order::Asc),
    "down_vote_desc" => (comments::Column::DownVote, Order::Desc),
    _ => (comments::Column::CreatedAt, Order::Desc),
  };

  // #[derive(FromQueryResult)]
  // struct CommentWithCount {
  //   total_count: i64,
  //   #[sea_orm(primary_key)]
  //   pub id: i64,
  //   pub user_id: Option<i64>,
  //   pub site_id: i64,
  //   pub rid: i64,
  //   pub page_path: String,
  //   pub content: String,
  //   pub status: CommentStatus,
  //   pub nickname: String,
  //   pub link: String,
  //   pub email: String,
  //   pub device: String,
  //   pub location: String,
  //   pub is_sticky: bool,
  //   pub up_vote: i32,
  //   pub down_vote: i32,
  //   pub created_at: DateTime,
  //   pub updated_at: DateTime,
  //   pub deleted_at: Option<DateTime>,
  // }

  // let query = match conn.get_database_backend() {
  //   DbBackend::Postgres => {
  //     r#"
  //     SELECT *, count(*) OVER() AS total_count
  //     FROM comments
  //     WHERE site_id = $1 AND page_path = $2 AND status != 'spam'
  //     ORDER BY created_at ASC
  //     LIMIT $3 OFFSET $4
  //     "#
  //   }
  //   _ => {
  //     r#"
  //     SELECT *, count(*) OVER() AS total_count
  //     FROM comments
  //     WHERE site_id = ? AND page_path = ? AND status != 'spam'
  //     ORDER BY created_at ASC
  //     LIMIT ? OFFSET ?
  //     "#
  //   }
  // };

  // let stmt = Statement::from_sql_and_values(
  //   conn.get_database_backend(),
  //   query,
  //   [
  //     qs.site_id.into(),
  //     qs.page_path.into(),
  //     qs.limit.into(),
  //     (qs.offset - 1).into(),
  //   ],
  // );

  // let comment = CommentWithCount::find_by_statement(stmt)
  //   .all(conn)
  //   .await
  //   .with_op("query comments")?;

  // let total = comment.first().map(|c| c.total_count).unwrap_or(0) as u64;
  // let total_page = (total as f64 / qs.limit as f64).ceil() as u64;
  // let page = qs.offset;
  // let page_size = qs.limit;

  // let items: Vec<CommentView> = comment
  //   .into_iter()
  //   .map(|c| CommentView {
  //     id: c.id,
  //     rid: c.rid,
  //     nickname: c.nickname,
  //     link: c.link,
  //     content: c.content,
  //     up_vote: c.up_vote,
  //     down_vote: c.down_vote,
  //     device: c.device,
  //     location: c.location,
  //     is_sticky: c.is_sticky,
  //     created_at: c.created_at.and_utc().to_rfc3339(),
  //   })
  //   .collect();

  let paginator = Comments::find()
    .filter(comments::Column::SiteId.eq(qs.site_id))
    .filter(comments::Column::PagePath.eq(qs.page_path))
    .filter(comments::Column::Status.is_not_in([CommentStatus::Spam]))
    .order_by(sort_col, sort_ord)
    .paginate(conn, qs.limit);

  let total = paginator.num_items().await.with_op("count comments")?;
  let total_page = (total as f64 / qs.limit as f64).ceil() as u64;
  let page = paginator.cur_page() + 1;
  let page_size = qs.limit;

  let comments = paginator
    .fetch_page(qs.offset - 1)
    .await
    .with_op("fetch comments")?;

  let items = comments
    .into_iter()
    .map(|comment| CommentView {
      id: comment.id,
      rid: comment.rid,
      nickname: comment.nickname,
      link: comment.link,
      content: comment.content,
      up_vote: comment.up_vote,
      down_vote: comment.down_vote,
      device: comment.device,
      location: comment.location,
      is_sticky: comment.is_sticky,
      created_at: comment.created_at.and_utc().to_rfc3339(),
    })
    .collect();

  Ok(PageResponse {
    items,
    page_size,
    page,
    total,
    total_page,
  })
}
