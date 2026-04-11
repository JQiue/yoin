use helpers::time::utc_now;

use super::AppService;
use crate::{
  error::{AppError, ToAppError},
  handler::{
    comment::CommentView,
    moderation::{
      CreateModerationProviderPayload, ModerationProviderView, UpdateModerationProviderPayload,
    },
  },
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
