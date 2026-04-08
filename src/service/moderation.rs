use super::AppService;
use crate::{
  error::{AppError, ToAppError},
  handler::{
    comment::CommentView,
    moderation::{
      CreateModerationProviderPayload, ModerationProviderView, UpdateModerationProviderPayload,
    },
  },
};

impl AppService {
  /// List moderation providers (TODO).
  pub async fn list_moderation_providers(&self) -> Result<Vec<ModerationProviderView>, AppError> {
    todo!()
  }

  /// Create a moderation provider (TODO).
  pub async fn create_moderation_provider(
    &self,
    _payload: CreateModerationProviderPayload,
  ) -> Result<ModerationProviderView, AppError> {
    todo!()
  }

  /// Update a moderation provider (TODO).
  pub async fn update_moderation_provider(
    &self,
    _id: i64,
    _payload: UpdateModerationProviderPayload,
  ) -> Result<ModerationProviderView, AppError> {
    todo!()
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
        .map(CommentView::from_model)
        .collect::<Vec<CommentView>>(),
    )
  }

  /// Approve a pending comment (TODO).
  pub async fn approve_comment(&self, _id: i64) -> Result<(), AppError> {
    todo!()
  }

  /// Reject a pending comment (TODO).
  pub async fn reject_comment(&self, _id: i64) -> Result<(), AppError> {
    todo!()
  }
}
