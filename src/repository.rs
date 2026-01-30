use migration::enums::CommentStatus;
use sea_orm::{
  ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, Order,
  PaginatorTrait, QueryFilter, QueryOrder, entity::prelude::*,
};

use crate::entity::{
  comments,
  prelude::{Comments, Sites, Users},
  sites::{self, SiteConfig},
  users::{self},
};

pub struct UserCreateData {
  pub nickname: String,
  pub password: String,
  pub email: String,
  pub url: String,
  pub avatar: String,
  pub role: migration::enums::UserRole,
  pub datetime: DateTime,
}

#[derive(Default)]
pub struct UserUpdateData {
  pub id: i64,
  pub nickname: Option<String>,
  pub password: Option<String>,
  pub email: Option<String>,
  pub url: Option<String>,
  pub avatar: Option<String>,
  pub role: Option<migration::enums::UserRole>,
  pub datetime: DateTime,
}

pub trait UserRepositoryTrait {
  async fn exists_any(&self) -> Result<bool, DbErr>;
  async fn exists_by_email(&self, email: &str) -> Result<bool, DbErr>;
  async fn find_by_id(&self, user_id: i64) -> Result<Option<users::Model>, DbErr>;
  async fn find_by_email(&self, email: &str) -> Result<Option<users::Model>, DbErr>;
  async fn find_all(&self) -> Result<Vec<users::Model>, DbErr>;
  async fn create(&self, data: UserCreateData) -> Result<users::Model, DbErr>;
  async fn update(&self, data: UserUpdateData) -> Result<users::Model, DbErr>;
}

pub struct UserRepository {
  pub conn: &'static DatabaseConnection,
}

impl UserRepositoryTrait for UserRepository {
  async fn exists_any(&self) -> Result<bool, DbErr> {
    Ok(Users::find().count(self.conn).await? == 0)
  }

  async fn exists_by_email(&self, email: &str) -> Result<bool, DbErr> {
    let user = Users::find()
      .filter(users::Column::Email.eq(email))
      .one(self.conn)
      .await?;
    Ok(user.is_some())
  }

  async fn find_by_id(&self, user_id: i64) -> Result<Option<users::Model>, DbErr> {
    Users::find()
      .filter(users::Column::Id.eq(user_id))
      .one(self.conn)
      .await
  }

  async fn find_by_email(&self, email: &str) -> Result<Option<users::Model>, DbErr> {
    Users::find()
      .filter(users::Column::Email.eq(email))
      .one(self.conn)
      .await
  }

  async fn find_all(&self) -> Result<Vec<users::Model>, DbErr> {
    Users::find().all(self.conn).await
  }

  async fn create(&self, data: UserCreateData) -> Result<users::Model, DbErr> {
    let insert_user = users::ActiveModel {
      nickname: Set(data.nickname),
      password: Set(data.password),
      email: Set(data.email),
      avatar: Set(data.avatar),
      role: Set(data.role),
      url: Set(data.url),
      created_at: Set(data.datetime),
      updated_at: Set(data.datetime),
      ..Default::default()
    };
    insert_user.insert(self.conn).await
  }

  async fn update(&self, data: UserUpdateData) -> Result<users::Model, DbErr> {
    let mut user = users::ActiveModel {
      id: Set(data.id),
      updated_at: Set(data.datetime),
      ..Default::default()
    };
    if let Some(nickname) = data.nickname {
      user.nickname = Set(nickname);
    }
    if let Some(password) = data.password {
      user.password = Set(password);
    }
    if let Some(email) = data.email {
      user.email = Set(email);
    }
    if let Some(url) = data.url {
      user.url = Set(url);
    }
    if let Some(avatar) = data.avatar {
      user.avatar = Set(avatar);
    }
    if let Some(role) = data.role {
      user.role = Set(role);
    }
    user.update(self.conn).await
  }
}

pub struct SiteCreateData {
  pub name: String,
  pub url: String,
  pub config: SiteConfig,
  pub datetime: DateTime,
}

pub struct SiteUpdateData {
  pub id: i64,
  pub name: Option<String>,
  pub url: Option<String>,
  pub config: Option<SiteConfig>,
  pub datetime: DateTime,
}

pub trait SiteRepositoryTrait {
  async fn create_default(&self, datetime: DateTime) -> Result<sites::Model, DbErr>;
  async fn create(&self, data: SiteCreateData) -> Result<sites::Model, DbErr>;
  async fn update(&self, data: SiteUpdateData) -> Result<sites::Model, DbErr>;
  async fn find_all(&self) -> Result<Vec<sites::Model>, DbErr>;
  async fn find_by_id(&self, id: i64) -> Result<Option<sites::Model>, DbErr>;
}

pub struct SiteRepository {
  pub conn: &'static DatabaseConnection,
}

impl SiteRepositoryTrait for SiteRepository {
  async fn find_all(&self) -> Result<Vec<sites::Model>, DbErr> {
    Sites::find().all(self.conn).await
  }

  async fn find_by_id(&self, id: i64) -> Result<Option<sites::Model>, DbErr> {
    Sites::find()
      .filter(sites::Column::Id.eq(id))
      .one(self.conn)
      .await
  }

  async fn create_default(&self, datetime: DateTime) -> Result<sites::Model, DbErr> {
    let new_site = sites::ActiveModel {
      name: Set("Default Site".to_string()),
      url: Set("".to_string()),
      config: Set(SiteConfig::default()),
      created_at: Set(datetime),
      updated_at: Set(datetime),
      ..Default::default()
    };
    new_site.insert(self.conn).await
  }

  async fn create(&self, data: SiteCreateData) -> Result<sites::Model, DbErr> {
    let new_site = sites::ActiveModel {
      name: Set(data.name),
      url: Set(data.url),
      config: Set(data.config),
      created_at: Set(data.datetime),
      updated_at: Set(data.datetime),
      ..Default::default()
    };
    new_site.insert(self.conn).await
  }

  async fn update(&self, data: SiteUpdateData) -> Result<sites::Model, DbErr> {
    let mut site = sites::ActiveModel {
      id: Set(data.id),
      updated_at: Set(data.datetime),
      ..Default::default()
    };

    if let Some(name) = data.name {
      site.name = Set(name);
    }

    if let Some(url) = data.url {
      site.url = Set(url);
    }

    if let Some(config) = data.config {
      site.config = Set(config);
    }

    site.update(self.conn).await
  }
}

pub struct CommentCreateData {
  pub site_id: i64,
  pub user_id: Option<i64>,
  pub thread_id: Option<i64>,
  pub parent_id: Option<i64>,
  pub nickname: String,
  pub page_path: String,
  pub link: String,
  pub content: String,
  pub email: String,
  pub device: String,
  pub location: String,
  pub is_sticky: bool,
  pub datetime: DateTime,
}

pub trait CommentRepositoryTrait {
  async fn create(&self, data: CommentCreateData) -> Result<comments::Model, DbErr>;
  async fn find_by_id(&self, id: i64) -> Result<Option<comments::Model>, DbErr>;
  async fn find_roots_paged(
    &self,
    site_id: i64,
    page_path: &str,
    page_size: u64,
    page_offset: u64,
    sort: &str,
  ) -> Result<(Vec<comments::Model>, u64, u64), DbErr>;
  async fn find_all_replies_by_thread_ids(
    &self,
    thread_ids: Vec<i64>,
  ) -> Result<Vec<comments::Model>, DbErr>;
}

pub struct CommentRepository {
  pub conn: &'static DatabaseConnection,
}

impl CommentRepositoryTrait for CommentRepository {
  async fn find_roots_paged(
    &self,
    site_id: i64,
    page_path: &str,
    page_size: u64,
    page_offset: u64,
    sort: &str,
  ) -> Result<(Vec<comments::Model>, u64, u64), DbErr> {
    let (sort_col, sort_ord) = match sort {
      "created_asc" => (comments::Column::CreatedAt, Order::Asc),
      "created_desc" => (comments::Column::CreatedAt, Order::Desc),
      "up_vote_asc" => (comments::Column::UpVote, Order::Asc),
      "up_vote_desc" => (comments::Column::UpVote, Order::Desc),
      "down_vote_asc" => (comments::Column::DownVote, Order::Asc),
      "down_vote_desc" => (comments::Column::DownVote, Order::Desc),
      _ => (comments::Column::CreatedAt, Order::Desc),
    };
    let paginator = Comments::find()
      .filter(comments::Column::SiteId.eq(site_id))
      .filter(comments::Column::PagePath.eq(page_path))
      .filter(comments::Column::Status.is_not_in([CommentStatus::Spam]))
      .filter(comments::Column::ParentId.is_null())
      .order_by(sort_col, sort_ord)
      .paginate(self.conn, page_size);
    let total = paginator.num_items().await?;
    let total_page = (total as f64 / page_size as f64).ceil() as u64;
    let page_idx = if page_offset > 0 { page_offset - 1 } else { 0 };
    let comments = paginator.fetch_page(page_idx).await?;
    Ok((comments, total, total_page))
  }

  async fn create(&self, data: CommentCreateData) -> Result<comments::Model, DbErr> {
    let mut active_comment = comments::ActiveModel {
      site_id: Set(data.site_id),
      nickname: Set(data.nickname),
      thread_id: Set(data.thread_id),
      parent_id: Set(data.parent_id),
      page_path: Set(data.page_path),
      link: Set(data.link),
      content: Set(data.content),
      email: Set(data.email),
      device: Set(data.device),
      location: Set(data.location),
      is_sticky: Set(data.is_sticky),
      created_at: Set(data.datetime),
      updated_at: Set(data.datetime),
      ..Default::default()
    };

    if let Some(user_id) = data.user_id {
      active_comment.user_id = Set(Some(user_id));
    }

    active_comment.insert(self.conn).await
  }

  async fn find_by_id(&self, id: i64) -> Result<Option<comments::Model>, DbErr> {
    Comments::find_by_id(id).one(self.conn).await
  }

  async fn find_all_replies_by_thread_ids(
    &self,
    thread_ids: Vec<i64>,
  ) -> Result<Vec<comments::Model>, DbErr> {
    Comments::find()
      .filter(comments::Column::ThreadId.is_in(thread_ids))
      .filter(comments::Column::ParentId.is_not_null())
      .all(self.conn)
      .await
  }
}

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
}
