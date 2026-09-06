use helpers::time::utc_now;
use migration::enums::CommentStatus;

use super::AppService;
use crate::{
  entity::{comments, moderation_providers::ModerationProviderConfig},
  error::{AppError, ToAppError},
  handler::comment::{CommentView, CreateCommentPayload, ListQueryString, PageResponse},
  helper::generate_avatar,
  moderation::{self, LLMModerator, ModerationInput, types::CommentModerator},
  rbac::permissions::codes::SITE_MANAGE,
  repository::{CommentCreateData, CommentListVisibility},
};

impl AppService {
  async fn can_manage_site(&self, user_id: Option<i64>, site_id: i64) -> Result<bool, AppError> {
    match user_id {
      Some(id) => self.has_site_permission(id, SITE_MANAGE, site_id).await,
      None => Ok(false),
    }
  }

  fn is_comment_author(
    comment: &comments::Model,
    user_id: Option<i64>,
    guest_id: Option<&str>,
  ) -> bool {
    if let (Some(author_id), Some(viewer_id)) = (comment.user_id, user_id)
      && author_id == viewer_id
    {
      return true;
    }
    matches!(
      (comment.guest_id.as_deref(), guest_id),
      (Some(author_guest), Some(viewer_guest)) if author_guest == viewer_guest
    )
  }

  pub(crate) async fn can_see_private_comment(
    &self,
    comment: &comments::Model,
    user_id: Option<i64>,
    guest_id: Option<&str>,
  ) -> Result<bool, AppError> {
    if !comment.is_private {
      return Ok(true);
    }
    if Self::is_comment_author(comment, user_id, guest_id) {
      return Ok(true);
    }
    self.can_manage_site(user_id, comment.site_id).await
  }

  async fn comment_list_visibility(
    &self,
    user_id: Option<i64>,
    guest_id: Option<&str>,
    site_id: i64,
  ) -> Result<CommentListVisibility, AppError> {
    Ok(CommentListVisibility {
      include_all_private: self.can_manage_site(user_id, site_id).await?,
      viewer_user_id: user_id,
      viewer_guest_id: guest_id.filter(|id| !id.is_empty()).map(str::to_string),
    })
  }

  /// Create a new comment or reply.
  ///
  /// If `payload.parent_id` is provided, the method validates the parent belongs to
  /// the same site/page and derives the thread id from it. The initial status is
  /// set based on whether an enabled moderation provider exists for the site.
  pub async fn create_comment(
    &self,
    user_id: Option<i64>,
    guest_id: Option<&str>,
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

      if parent.site_id != payload.site_id || parent.page_path != payload.page_path {
        return Err(AppError::bad_request(
          "Parent comment does not belong to the current site or page".to_string(),
        ));
      }

      if matches!(parent.status, CommentStatus::Deleted | CommentStatus::Spam) {
        return Err(AppError::bad_request(
          "Parent comment is not available for reply".to_string(),
        ));
      }

      if !self
        .can_see_private_comment(&parent, user_id, guest_id)
        .await?
      {
        return Err(AppError::comment_not_found("Comment not found".to_string()));
      }

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
    let moderation_provider = self
      .repo
      .moderation_provider()
      .find_enabled_by_site(payload.site_id)
      .await
      .with_op("find enabled moderation provider")?;
    let initial_status = if moderation_provider.is_some() {
      CommentStatus::Pending
    } else {
      CommentStatus::Approved
    };
    let comment = self
      .repo
      .comment()
      .create(CommentCreateData {
        site_id: payload.site_id,
        user_id,
        guest_id: if user_id.is_none() {
          guest_id.filter(|id| !id.is_empty()).map(str::to_string)
        } else {
          None
        },
        thread_id,
        parent_id: payload.parent_id,
        nickname,
        page_path: payload.page_path,
        website: payload.website,
        content: payload.content.clone(),
        email: payload.email.clone(),
        avatar,
        device,
        status: initial_status,
        location,
        is_sticky: false,
        is_anonymous: payload.is_anonymous,
        is_private: payload.is_private,
        datetime: utc_now().naive_utc(),
      })
      .await
      .with_op("insert comment")?;

    if let Some(moderation_provider) = moderation_provider {
      let comment_repo = self.repo.comment();
      let moderation_input = ModerationInput {
        content: payload.content.clone(),
        email: payload.email.clone(),
      };
      tokio::spawn(async move {
        let status = match moderation_provider.config {
          ModerationProviderConfig::LLM {
            model,
            api_base,
            api_key,
            rule,
          } => {
            match LLMModerator::new(api_base, api_key, model, Some(rule))
              .check(moderation_input)
              .await
            {
              Ok(result) => {
                tracing::info!(
                  provider = result.provider,
                  ?result.decision,
                  score = ?result.score,
                  reason = ?result.reason,
                  "comment moderation result"
                );
                match result.decision {
                  moderation::ModerationDecision::Allow => CommentStatus::Approved,
                  moderation::ModerationDecision::Review => CommentStatus::Pending,
                  moderation::ModerationDecision::Reject => CommentStatus::Spam,
                }
              }
              Err(error) => {
                tracing::error!(?error, "comment moderation failed");
                CommentStatus::Pending
              }
            }
          }
          ModerationProviderConfig::AKISMET { .. } => {
            tracing::warn!(
              site_id = moderation_provider.site_id,
              "akismet moderation provider configured but not implemented"
            );
            CommentStatus::Pending
          }
        };

        if let Err(e) = comment_repo
          .update_status_if_pending(comment.id, status)
          .await
          .with_op("update comment status")
        {
          tracing::error!(?e, "failed to update comment status");
        };
      });
    }

    let reveal_identity = self.can_manage_site(user_id, comment.site_id).await?;
    Ok(CommentView::from_model(comment, reveal_identity))
  }

  /// List root comments for a site/page with pagination.
  ///
  /// For each root comment, this also includes up to `reply_limit` preview replies
  /// per thread (and sets `has_more` accordingly).
  pub async fn list_comments(
    &self,
    user_id: Option<i64>,
    guest_id: Option<&str>,
    qs: ListQueryString,
  ) -> Result<PageResponse<CommentView>, AppError> {
    let visibility = self
      .comment_list_visibility(user_id, guest_id, qs.site_id)
      .await?;
    let reveal_identity = visibility.include_all_private;
    let (roots, total, total_pages) = self
      .repo
      .comment()
      .find_roots_paged(
        qs.site_id,
        &qs.page_path,
        qs.page_size,
        qs.page_offset,
        &qs.sort,
        &visibility,
      )
      .await
      .with_op("query comments")?;
    let root_ids: Vec<i64> = roots.iter().map(|c| c.id).collect();
    let reply_limit = 3;
    let preview_replies = self
      .repo
      .comment()
      .find_preview_replies_by_thread_ids(root_ids, reply_limit, &visibility)
      .await
      .with_op("find all replies by thread ids")?;
    let mut items: Vec<CommentView> = roots
      .into_iter()
      .map(|root| {
        let mut view = CommentView::from_model(root, reveal_identity);
        let mut thread_replies: Vec<CommentView> = preview_replies
          .iter()
          .filter(|r| r.thread_id == Some(view.id))
          .map(|r| CommentView::from_model(r.clone(), reveal_identity))
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
    let actor_id = match user_id {
      Some(id) => Some(id.to_string()),
      None => guest_id.map(str::to_string),
    };
    self
      .attach_comment_reactions(&mut items, actor_id.as_deref())
      .await?;

    Ok(PageResponse {
      items,
      page_size: qs.page_size,
      page_offset: qs.page_offset,
      total_pages,
      total,
    })
  }

  /// List replies under a specific thread (comment id) with pagination.
  pub async fn list_replies(
    &self,
    user_id: Option<i64>,
    guest_id: Option<&str>,
    id: i64,
    qs: ListQueryString,
  ) -> Result<PageResponse<CommentView>, AppError> {
    let visibility = self
      .comment_list_visibility(user_id, guest_id, qs.site_id)
      .await?;
    let reveal_identity = visibility.include_all_private;
    let (replies, total, total_pages) = self
      .repo
      .comment()
      .find_thread_replies_paged(
        id,
        qs.site_id,
        &qs.page_path,
        qs.page_size,
        qs.page_offset,
        &visibility,
      )
      .await
      .with_op("query replies")?;
    let mut items: Vec<CommentView> = replies
      .into_iter()
      .map(|comment| CommentView::from_model(comment, reveal_identity))
      .collect();
    let actor_id = match user_id {
      Some(id) => Some(id.to_string()),
      None => guest_id.map(str::to_string),
    };
    self
      .attach_comment_reactions(&mut items, actor_id.as_deref())
      .await?;
    Ok(PageResponse {
      items,
      page_size: qs.page_size,
      page_offset: qs.page_offset,
      total_pages,
      total,
    })
  }

  /// Soft-delete a comment.
  ///
  /// Only the comment owner can delete it. If the comment is a root comment
  /// (`parent_id` is `None`), the whole thread is deleted; otherwise, only the
  /// comment is soft-deleted.
  pub async fn delete_comment(&self, user_id: i64, id: i64) -> Result<(), AppError> {
    let comment = self
      .repo
      .comment()
      .find_by_id(id)
      .await
      .with_op("find comment by id")?
      .ok_or(AppError::comment_not_found("Comment not found".to_string()))?;

    if comment.user_id != Some(user_id) {
      return Err(AppError::forbidden(
        "You are not the owner of this comment".to_string(),
      ));
    }

    if comment.parent_id.is_none() {
      self
        .repo
        .comment()
        .soft_delete_thread(comment.id)
        .await
        .with_op("delete comment thread")?;
    } else {
      self
        .repo
        .comment()
        .soft_delete(comment)
        .await
        .with_op("delete comment")?;
    }

    Ok(())
  }
}
