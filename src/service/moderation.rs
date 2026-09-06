use helpers::time::utc_now;
use migration::enums::CommentStatus;

use super::AppService;
use crate::{
  error::{AppError, ToAppError},
  handler::{
    comment::CommentView,
    moderation::{
      CreateModerationProviderPayload, ModerationProviderView, UpdateModerationProviderPayload,
    },
  },
  rbac::permissions::codes::COMMENT_MODERATE,
  repository::{ModerationProviderCreateData, ModerationProviderUpdateData},
};

impl AppService {
  /// List moderation providers (TODO).
  pub async fn list_moderation_providers(&self) -> Result<Vec<ModerationProviderView>, AppError> {
    let providers = self
      .repo
      .moderation_provider()
      .find_all()
      .await
      .with_op("find all moderation providers")?;
    Ok(
      providers
        .into_iter()
        .map(ModerationProviderView::from_model)
        .collect(),
    )
  }

  /// Create a moderation provider (TODO).
  pub async fn create_moderation_provider(
    &self,
    payload: CreateModerationProviderPayload,
  ) -> Result<ModerationProviderView, AppError> {
    self
      .repo
      .site()
      .find_by_id(payload.site_id)
      .await
      .with_op("find site for moderation provider")?
      .ok_or_else(|| AppError::site_not_found("site not found".to_string()))?;

    let provider = self
      .repo
      .moderation_provider()
      .create(ModerationProviderCreateData {
        site_id: payload.site_id,
        provider_kind: payload.provider_kind,
        enabled: payload.enabled,
        config: payload.config,
        datetime: utc_now().naive_local(),
      })
      .await
      .with_op("insert moderation provider")?;
    Ok(ModerationProviderView::from_model(provider))
  }

  /// Update a moderation provider (TODO).
  pub async fn update_moderation_provider(
    &self,
    id: i64,
    payload: UpdateModerationProviderPayload,
  ) -> Result<ModerationProviderView, AppError> {
    self
      .repo
      .moderation_provider()
      .find_by_id(id)
      .await
      .with_op("find moderation provider")?
      .ok_or_else(|| {
        AppError::moderation_provider_not_found("moderation provider not found".to_string())
      })?;

    let provider = self
      .repo
      .moderation_provider()
      .update(ModerationProviderUpdateData {
        id,
        enabled: payload.enabled,
        config: payload.config,
        datetime: utc_now().naive_local(),
      })
      .await
      .with_op("update moderation provider")?;
    Ok(ModerationProviderView::from_model(provider))
  }

  /// List comments that are pending moderation.
  pub async fn list_pending_comments(&self) -> Result<Vec<CommentView>, AppError> {
    Ok(
      self
        .repo
        .comment()
        .find_pending()
        .await
        .with_op("find pending comments")?
        .into_iter()
        .map(|comment| CommentView::from_model(comment, true, false, false))
        .collect::<Vec<CommentView>>(),
    )
  }

  /// Approve a pending comment.
  pub async fn approve_comment(&self, user_id: i64, id: i64) -> Result<(), AppError> {
    self
      .set_pending_comment_status(user_id, id, CommentStatus::Approved)
      .await
  }

  /// Reject a pending comment.
  pub async fn reject_comment(&self, user_id: i64, id: i64) -> Result<(), AppError> {
    self
      .set_pending_comment_status(user_id, id, CommentStatus::Spam)
      .await
  }

  async fn set_pending_comment_status(
    &self,
    user_id: i64,
    id: i64,
    status: CommentStatus,
  ) -> Result<(), AppError> {
    let comment = self
      .repo
      .comment()
      .find_by_id(id)
      .await
      .with_op("find comment by id")?
      .ok_or(AppError::comment_not_found("Comment not found".to_string()))?;
    self
      .require_site_permission(user_id, COMMENT_MODERATE, comment.site_id)
      .await?;
    if comment.status != CommentStatus::Pending {
      return Err(AppError::bad_request(
        "only pending comments can be approved or rejected".to_string(),
      ));
    }
    self
      .repo
      .comment()
      .update_status(id, status)
      .await
      .with_op("update comment status")?;
    Ok(())
  }
}
