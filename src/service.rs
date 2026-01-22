use helpers::{
  hash::{argon2, verify_argon2},
  jwt,
  time::utc_now,
  uuid::{Alphabet, nanoid},
};
use migration::enums::UserRole;

use crate::{
  error::{AppError, ToAppError},
  handler::{
    auth::{LoginPayload, UserProfile, UserWithToken},
    comment::{CommentView, CreateCommentPayload, ListQueryString, PageResponse},
    site::{CreateSitePayload, SiteView, UpdateSitePayload},
    user::UpdateProfilePayload,
  },
  helper::generate_avatar,
  repository::{
    CommentCreateData, CommentRepositoryTrait, Repository, SiteCreateData, SiteRepositoryTrait,
    SiteUpdateData, UserCreateData, UserRepositoryTrait, UserUpdateData,
  },
};

pub struct AppService {
  pub repo: Repository,
}

impl AppService {
  pub async fn create_user(
    &self,
    nickname: String,
    url: String,
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
    let role = if self
      .repo
      .user()
      .exists_any()
      .await
      .with_op("is first user")?
    {
      self
        .repo
        .site()
        .create_default(datetime)
        .await
        .with_op("create default site")?;
      UserRole::Admin
    } else {
      UserRole::Normal
    };
    let hashed = argon2(&password, &nanoid(&Alphabet::DEFAULT, 8)).with_op("hash_password")?;
    let new_user = self
      .repo
      .user()
      .create(UserCreateData {
        nickname,
        password: hashed,
        email: email.clone(),
        url,
        avatar: generate_avatar(&email),
        role,
        datetime,
      })
      .await
      .with_op("insert_user")?;
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
        url: user.url,
        role: user.role.to_string(),
      },
      // datetime: user.created_at.and_utc().to_rfc3339(),
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
      url: user.url,
      role: user.role.to_string(),
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
        url: payload.url,
        avatar: payload.avatar,
        datetime: utc_now().naive_utc(),
        ..Default::default()
      })
      .await
      .with_op("update_user")?;
    Ok(UserProfile {
      avatar: updated_user.avatar,
      nickname: updated_user.nickname,
      url: updated_user.url,
      role: updated_user.role.to_string(),
    })
  }

  pub async fn list_sites(&self, user_id: i64) -> Result<Vec<SiteView>, AppError> {
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

    let sites = self
      .repo
      .site()
      .find_all()
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
    &self,
    user_id: i64,
    payload: CreateSitePayload,
  ) -> Result<SiteView, AppError> {
    let user = self
      .repo
      .user()
      .find_by_id(user_id)
      .await
      .with_op("find user by id")?
      .ok_or(AppError::user_not_found("User not found".to_string()))?;

    if user.role != UserRole::Admin {
      return Err(AppError::forbidden(
        "You are not admin".to_string() + &user.id.to_string(),
      ));
    }
    let datetime = utc_now().naive_utc();
    let site = self
      .repo
      .site()
      .create(SiteCreateData {
        name: payload.name,
        url: payload.url,
        config: payload.config,
        datetime,
      })
      .await
      .with_op("insert_site")?;
    Ok(SiteView {
      id: site.id,
      name: site.name,
      config: site.config,
      url: site.url,
    })
  }

  pub async fn update_site(
    &self,
    user_id: i64,
    payload: UpdateSitePayload,
  ) -> Result<SiteView, AppError> {
    let user = self
      .repo
      .user()
      .find_by_id(user_id)
      .await
      .with_op("find user by id")?
      .ok_or(AppError::user_not_found("User not found".to_string()))?;

    if user.role != UserRole::Admin {
      return Err(AppError::forbidden(
        "You are not admin".to_string() + &user.id.to_string(),
      ));
    }

    let site = self
      .repo
      .site()
      .find_by_id(payload.id)
      .await
      .with_op("find site by id")?
      .ok_or(AppError::site_not_found("Site not found".to_string()))?;

    let site = self
      .repo
      .site()
      .update(SiteUpdateData {
        id: site.id,
        name: payload.name,
        url: payload.url,
        config: payload.config,
        datetime: utc_now().naive_utc(),
      })
      .await
      .with_op("update site")?;

    Ok(SiteView {
      id: site.id,
      name: site.name,
      config: site.config,
      url: site.url,
    })
  }

  pub async fn create_comment(
    &self,
    user_id: Option<i64>,
    payload: CreateCommentPayload,
  ) -> Result<CommentView, AppError> {
    let device = "unknown".to_string();
    let location = "unknown".to_string();
    let comment = self
      .repo
      .comment()
      .create(CommentCreateData {
        site_id: payload.site_id,
        user_id,
        rid: payload.rid,
        nickname: payload.nickname,
        page_path: payload.page_path,
        link: payload.link,
        content: payload.content,
        email: payload.email,
        device: device.clone(),
        location: location.clone(),
        is_sticky: false,
        datetime: utc_now().naive_utc(),
      })
      .await
      .with_op("insert comment")?;

    Ok(CommentView {
      id: comment.id,
      rid: comment.rid,
      nickname: comment.nickname,
      link: comment.link,
      content: comment.content,
      up_vote: comment.up_vote,
      down_vote: comment.down_vote,
      device,
      location,
      is_sticky: comment.is_sticky,
      created_at: comment.created_at.and_utc().to_rfc3339(),
    })
  }

  pub async fn list_comments(
    &self,
    qs: ListQueryString,
  ) -> Result<PageResponse<CommentView>, AppError> {
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
    let (comments, total, total_page) = self
      .repo
      .comment()
      .find_paged(
        qs.site_id,
        &qs.page_path,
        qs.page_size,
        qs.page_offset,
        &qs.sort,
      )
      .await
      .with_op("query comments")?;

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
      page_size: qs.page_size,
      page_offset: qs.page_offset,
      total_page,
      total,
    })
  }
}
