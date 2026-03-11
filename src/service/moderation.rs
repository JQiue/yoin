use super::AppService;
use crate::{
  error::AppError,
  handler::{
    comment::CommentView,
    moderation::{
      CreateModerationProviderPayload, ModerationProviderView, UpdateModerationProviderPayload,
    },
  },
};

impl AppService {
  pub async fn list_moderation_providers(&self) -> Result<Vec<ModerationProviderView>, AppError> {
    todo!()
  }

  pub async fn create_moderation_provider(
    &self,
    _payload: CreateModerationProviderPayload,
  ) -> Result<ModerationProviderView, AppError> {
    todo!()
  }

  pub async fn update_moderation_provider(
    &self,
    _id: i64,
    _payload: UpdateModerationProviderPayload,
  ) -> Result<ModerationProviderView, AppError> {
    todo!()
  }

  pub async fn list_pending_comments(&self) -> Result<Vec<CommentView>, AppError> {
    todo!()
  }

  pub async fn approve_comment(&self, _id: i64) -> Result<(), AppError> {
    todo!()
  }

  pub async fn reject_comment(&self, _id: i64) -> Result<(), AppError> {
    todo!()
  }
}
