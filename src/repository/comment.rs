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
  pub guest_id: Option<String>,
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

#[derive(Clone, Debug)]
pub struct CommentListVisibility {
  pub include_all_private: bool,
  pub viewer_user_id: Option<i64>,
  pub viewer_guest_id: Option<String>,
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
    if let Some(guest_id) = data.guest_id {
      active_comment.guest_id = Set(Some(guest_id));
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

  pub async fn set_sticky(
    &self,
    comment: comments::Model,
    is_sticky: bool,
  ) -> Result<comments::Model, DbErr> {
    let mut active: comments::ActiveModel = comment.into();
    active.is_sticky = Set(is_sticky);
    active.updated_at = Set(utc_now().naive_utc());
    active.update(self.conn).await
  }

  fn apply_visibility(
    query: Select<Comments>,
    visibility: &CommentListVisibility,
  ) -> Select<Comments> {
    if visibility.include_all_private {
      return query;
    }

    let mut visible = Condition::any().add(comments::Column::IsPrivate.eq(false));
    if let Some(user_id) = visibility.viewer_user_id {
      visible = visible.add(
        Condition::all()
          .add(comments::Column::IsPrivate.eq(true))
          .add(comments::Column::UserId.eq(user_id)),
      );
    }
    if let Some(guest_id) = &visibility.viewer_guest_id {
      visible = visible.add(
        Condition::all()
          .add(comments::Column::IsPrivate.eq(true))
          .add(comments::Column::GuestId.eq(guest_id)),
      );
    }
    query.filter(visible)
  }

  pub async fn find_roots_paged(
    &self,
    site_id: i64,
    page_path: &str,
    page_size: u64,
    page_offset: u64,
    sort: &str,
    visibility: &CommentListVisibility,
  ) -> Result<(Vec<comments::Model>, u64, u64), DbErr> {
    let (sort_col, sort_ord) = match sort {
      "created_asc" => (comments::Column::CreatedAt, Order::Asc),
      "created_desc" => (comments::Column::CreatedAt, Order::Desc),
      _ => (comments::Column::CreatedAt, Order::Desc),
    };
    let paginator = Self::apply_visibility(
      Comments::find()
        .filter(comments::Column::SiteId.eq(site_id))
        .filter(comments::Column::PagePath.eq(page_path))
        .filter(comments::Column::Status.is_in([CommentStatus::Approved]))
        .filter(comments::Column::ParentId.is_null()),
      visibility,
    )
    .order_by_desc(comments::Column::IsSticky)
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
    visibility: &CommentListVisibility,
  ) -> Result<Vec<comments::Model>, DbErr> {
    let all = Self::apply_visibility(
      Comments::find()
        .filter(comments::Column::ThreadId.is_in(thread_ids))
        .filter(comments::Column::ParentId.is_not_null())
        .filter(comments::Column::Status.is_in([CommentStatus::Approved])),
      visibility,
    )
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
    visibility: &CommentListVisibility,
  ) -> Result<(Vec<comments::Model>, u64, u64), DbErr> {
    let paginator = Self::apply_visibility(
      Comments::find()
        .filter(comments::Column::SiteId.eq(site_id))
        .filter(comments::Column::ThreadId.eq(thread_id))
        .filter(comments::Column::PagePath.eq(page_path))
        .filter(comments::Column::Status.is_in([CommentStatus::Approved])),
      visibility,
    )
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
}
