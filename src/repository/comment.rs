use migration::enums::CommentStatus;
use sea_orm::{
  ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, Order,
  PaginatorTrait, QueryFilter, QueryOrder, entity::prelude::*,
};

use crate::entity::{comments, prelude::Comments};

pub struct CommentCreateData {
  pub site_id: i64,
  pub user_id: Option<i64>,
  pub thread_id: Option<i64>,
  pub parent_id: Option<i64>,
  pub nickname: String,
  pub page_path: String,
  pub website: String,
  pub avatar: String,
  pub content: String,
  pub email: String,
  pub device: String,
  pub location: String,
  pub is_sticky: bool,
  pub datetime: DateTime,
}

pub struct CommentRepository {
  pub conn: &'static DatabaseConnection,
}

impl CommentRepository {
  pub async fn find_roots_paged(
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
    let total_pages = (total as f64 / page_size as f64).ceil() as u64;
    let page_idx = if page_offset > 0 { page_offset - 1 } else { 0 };
    let comments = paginator.fetch_page(page_idx).await?;
    Ok((comments, total, total_pages))
  }

  pub async fn create(&self, data: CommentCreateData) -> Result<comments::Model, DbErr> {
    let mut active_comment = comments::ActiveModel {
      site_id: Set(data.site_id),
      nickname: Set(data.nickname),
      thread_id: Set(data.thread_id),
      parent_id: Set(data.parent_id),
      page_path: Set(data.page_path),
      website: Set(data.website),
      content: Set(data.content),
      email: Set(data.email),
      device: Set(data.device),
      location: Set(data.location),
      avatar: Set(data.avatar),
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

  pub async fn find_by_id(&self, id: i64) -> Result<Option<comments::Model>, DbErr> {
    Comments::find_by_id(id).one(self.conn).await
  }

  pub async fn find_all_replies_by_thread_ids(
    &self,
    thread_ids: Vec<i64>,
  ) -> Result<Vec<comments::Model>, DbErr> {
    Comments::find()
      .filter(comments::Column::ThreadId.is_in(thread_ids))
      .filter(comments::Column::ParentId.is_not_null())
      .all(self.conn)
      .await
  }

  pub async fn find_replies_by_thread(
    &self,
    thread_id: i64,
    site_id: i64,
    page_path: &str,
    page_size: u64,
    page_offset: u64,
  ) -> Result<(Vec<comments::Model>, u64, u64), DbErr> {
    let paginator = Comments::find()
      .filter(comments::Column::SiteId.eq(site_id))
      .filter(comments::Column::ThreadId.eq(thread_id))
      .filter(comments::Column::PagePath.eq(page_path))
      .filter(comments::Column::Status.is_not_in([CommentStatus::Spam]))
      .order_by(comments::Column::CreatedAt, Order::Asc)
      .paginate(self.conn, page_size);
    let total = paginator.num_items().await?;
    let total_pages = (total as f64 / page_size as f64).ceil() as u64;
    let page_idx = if page_offset > 0 { page_offset - 1 } else { 0 };
    let comments = paginator.fetch_page(page_idx).await?;
    Ok((comments, total, total_pages))
  }
}
