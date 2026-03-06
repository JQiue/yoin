use helpers::time::utc_now;

use super::AppService;
use crate::{
  error::{AppError, ToAppError},
  handler::comment::{CommentView, CreateCommentPayload, ListQueryString, PageResponse},
  helper::generate_avatar,
  repository::CommentCreateData,
};

impl AppService {
  pub async fn create_comment(
    &self,
    user_id: Option<i64>,
    payload: CreateCommentPayload,
  ) -> Result<CommentView, AppError> {
    let thread_id = if let Some(parent_id) = payload.parent_id {
      let parent = self
        .repo
        .comment()
        .find_by_id(parent_id)
        .await
        .with_op("find comment by id")?
        .ok_or(AppError::comment_not_found("Comment not found".to_string()))?;
      Some(parent.thread_id.unwrap_or(parent.id))
    } else {
      None
    };
    let device = "unknown".to_string();
    let location = "unknown".to_string();
    let (nickname, avatar) = if let Some(user_id) = user_id {
      let user = self
        .repo
        .user()
        .find_by_id(user_id)
        .await
        .with_op("find user by id")?
        .ok_or(AppError::user_not_found("User not found".to_string()))?;
      (user.nickname, user.avatar)
    } else {
      (payload.nickname, generate_avatar(&payload.email))
    };
    let comment = self
      .repo
      .comment()
      .create(CommentCreateData {
        site_id: payload.site_id,
        user_id,
        thread_id,
        parent_id: payload.parent_id,
        nickname,
        page_path: payload.page_path,
        website: payload.website,
        content: payload.content,
        email: payload.email,
        avatar,
        device,
        location,
        is_sticky: false,
        datetime: utc_now().naive_utc(),
      })
      .await
      .with_op("insert comment")?;

    Ok(CommentView {
      id: comment.id,
      thread_id: comment.thread_id,
      parent_id: comment.parent_id,
      nickname: comment.nickname,
      website: comment.website,
      content: comment.content,
      up_vote: comment.up_vote,
      down_vote: comment.down_vote,
      device: comment.device,
      location: comment.location,
      avatar: comment.avatar,
      is_sticky: comment.is_sticky,
      created_at: comment.created_at.and_utc().to_rfc3339(),
      replies: None,
      has_more: None,
    })
  }

  pub async fn list_comments(
    &self,
    qs: ListQueryString,
  ) -> Result<PageResponse<CommentView>, AppError> {
    let (roots, total, total_pages) = self
      .repo
      .comment()
      .find_roots_paged(
        qs.site_id,
        &qs.page_path,
        qs.page_size,
        qs.page_offset,
        &qs.sort,
      )
      .await
      .with_op("query comments")?;
    let root_ids: Vec<i64> = roots.iter().map(|c| c.id).collect();
    let all_replies = self
      .repo
      .comment()
      .find_all_replies_by_thread_ids(root_ids)
      .await
      .with_op("find all replies by thread ids")?;
    let reply_limit = 3;
    let items = roots
      .into_iter()
      .map(|root| {
        let mut view = CommentView::from_model(root);
        let mut thread_replies: Vec<CommentView> = all_replies
          .iter()
          .filter(|r| r.thread_id == Some(view.id))
          .map(|r| CommentView::from_model(r.clone()))
          .collect();

        if thread_replies.len() > reply_limit {
          view.has_more = Some(true);
          thread_replies.truncate(reply_limit);
        } else {
          view.has_more = Some(false);
        }

        view.replies = if thread_replies.is_empty() {
          None
        } else {
          Some(thread_replies)
        };
        view
      })
      .collect();

    Ok(PageResponse {
      items,
      page_size: qs.page_size,
      page_offset: qs.page_offset,
      total_pages,
      total,
    })
  }

  pub async fn list_replies(
    &self,
    id: i64,
    qs: ListQueryString,
  ) -> Result<PageResponse<CommentView>, AppError> {
    let (replies, total, total_pages) = self
      .repo
      .comment()
      .find_replies_by_thread(id, qs.site_id, &qs.page_path, qs.page_size, qs.page_offset)
      .await
      .with_op("query replies")?;
    let items: Vec<CommentView> = replies.into_iter().map(CommentView::from_model).collect();
    Ok(PageResponse {
      items,
      page_size: qs.page_size,
      page_offset: qs.page_offset,
      total_pages,
      total,
    })
  }
}
