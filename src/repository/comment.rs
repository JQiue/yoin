use helpers::time::utc_now;
use migration::enums::CommentStatus;
use sea_orm::{
  ActiveModelTrait, ActiveValue::Set, ColumnTrait, Condition, DatabaseConnection, DbErr,
  EntityTrait, Order, PaginatorTrait, QueryFilter, QueryOrder, UpdateResult, entity::prelude::*,
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
  pub is_anonymous: bool,
  pub is_private: bool,
  pub status: CommentStatus,
  pub datetime: DateTime,
}

pub struct CommentRepository {
  pub conn: &'static DatabaseConnection,
}

impl CommentRepository {
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
      is_anonymous: Set(data.is_anonymous),
      is_private: Set(data.is_private),
      status: Set(data.status),
      created_at: Set(data.datetime),
      updated_at: Set(data.datetime),
      ..Default::default()
    };

    if let Some(user_id) = data.user_id {
      active_comment.user_id = Set(Some(user_id));
    }

    active_comment.insert(self.conn).await
  }

  pub async fn find_all_paged(
    &self,
    page_size: u64,
    page_offset: u64,
    sort: &str,
    status: Option<CommentStatus>,
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
    let mut paginator = Comments::find();

    if let Some(status) = status {
      paginator = paginator.filter(comments::Column::Status.eq(status));
    }

    let paginator = paginator
      .order_by(sort_col, sort_ord)
      .paginate(self.conn, page_size);
    let total = paginator.num_items().await?;
    let total_pages = (total as f64 / page_size as f64).ceil() as u64;
    let page_idx = if page_offset > 0 { page_offset - 1 } else { 0 };
    let comments = paginator.fetch_page(page_idx).await?;
    Ok((comments, total, total_pages))
  }

  pub async fn find_by_id(&self, id: i64) -> Result<Option<comments::Model>, DbErr> {
    Comments::find_by_id(id).one(self.conn).await
  }

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
      .filter(comments::Column::Status.is_in([CommentStatus::Approved]))
      .filter(comments::Column::ParentId.is_null())
      .order_by(sort_col, sort_ord)
      .paginate(self.conn, page_size);
    let total = paginator.num_items().await?;
    let total_pages = (total as f64 / page_size as f64).ceil() as u64;
    let page_idx = if page_offset > 0 { page_offset - 1 } else { 0 };
    let comments = paginator.fetch_page(page_idx).await?;
    Ok((comments, total, total_pages))
  }

  pub async fn find_preview_replies_by_thread_ids(
    &self,
    thread_ids: Vec<i64>,
    per_thread_limit: usize,
  ) -> Result<Vec<comments::Model>, DbErr> {
    let all = Comments::find()
      .filter(comments::Column::ThreadId.is_in(thread_ids))
      .filter(comments::Column::ParentId.is_not_null())
      .filter(comments::Column::Status.is_in([CommentStatus::Approved]))
      .order_by(comments::Column::ThreadId, Order::Asc)
      .order_by(comments::Column::CreatedAt, Order::Asc)
      .all(self.conn)
      .await?;

    let mut counts = std::collections::HashMap::<i64, usize>::new();
    let mut preview = Vec::new();
    let limit = per_thread_limit + 1;

    for comment in all {
      let Some(thread_id) = comment.thread_id else {
        continue;
      };

      let count = counts.entry(thread_id).or_insert(0);
      if *count < limit {
        preview.push(comment);
        *count += 1;
      }
    }

    Ok(preview)
  }

  pub async fn find_thread_replies_paged(
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
      .filter(comments::Column::Status.is_in([CommentStatus::Approved]))
      .order_by(comments::Column::CreatedAt, Order::Asc)
      .paginate(self.conn, page_size);
    let total = paginator.num_items().await?;
    let total_pages = (total as f64 / page_size as f64).ceil() as u64;
    let page_idx = if page_offset > 0 { page_offset - 1 } else { 0 };
    let comments = paginator.fetch_page(page_idx).await?;
    Ok((comments, total, total_pages))
  }

  pub async fn find_pending(&self) -> Result<Vec<comments::Model>, DbErr> {
    Comments::find()
      .filter(comments::Column::Status.eq(CommentStatus::Pending))
      .all(self.conn)
      .await
  }

  pub async fn soft_delete(&self, comment: comments::Model) -> Result<comments::Model, DbErr> {
    comments::ActiveModel {
      id: Set(comment.id),
      status: Set(CommentStatus::Deleted),
      deleted_at: Set(Some(utc_now().naive_utc())),
      ..Default::default()
    }
    .update(self.conn)
    .await
  }

  pub async fn soft_delete_thread(&self, root_id: i64) -> Result<UpdateResult, DbErr> {
    Comments::update_many()
      .filter(
        Condition::any()
          .add(comments::Column::ThreadId.eq(root_id))
          .add(comments::Column::Id.eq(root_id)),
      )
      .set(comments::ActiveModel {
        status: Set(CommentStatus::Deleted),
        deleted_at: Set(Some(utc_now().naive_utc())),
        ..Default::default()
      })
      .exec(self.conn)
      .await
  }

  pub async fn update_status(&self, id: i64, status: CommentStatus) -> Result<UpdateResult, DbErr> {
    Comments::update_many()
      .filter(comments::Column::Id.eq(id))
      .set(comments::ActiveModel {
        status: Set(status),
        ..Default::default()
      })
      .exec(self.conn)
      .await
  }

  pub async fn update_status_if_pending(
    &self,
    id: i64,
    status: CommentStatus,
  ) -> Result<UpdateResult, DbErr> {
    Comments::update_many()
      .filter(comments::Column::Id.eq(id))
      .filter(comments::Column::Status.eq(CommentStatus::Pending))
      .set(comments::ActiveModel {
        status: Set(status),
        ..Default::default()
      })
      .exec(self.conn)
      .await
  }

  pub async fn update_up_vote(&self, id: i64, up_vote: i32) -> Result<comments::Model, DbErr> {
    Comments::update(comments::ActiveModel {
      id: Set(id),
      up_vote: Set(up_vote),
      ..Default::default()
    })
    .exec(self.conn)
    .await
  }

  pub async fn update_down_vote(&self, id: i64, down_vote: i32) -> Result<comments::Model, DbErr> {
    Comments::update(comments::ActiveModel {
      id: Set(id),
      down_vote: Set(down_vote),
      ..Default::default()
    })
    .exec(self.conn)
    .await
  }
}
