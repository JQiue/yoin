use super::AppService;
use crate::{
  error::AppError,
  handler::subscription::{CommentSubscriptionView, CreateCommentSubscriptionPayload},
};

impl AppService {
  /// List comment subscriptions for the given user.
  pub async fn list_comment_subscriptions(
    &self,
    _user_id: i64,
  ) -> Result<Vec<CommentSubscriptionView>, AppError> {
    todo!()
  }

  /// Create a comment subscription for the given user.
  pub async fn create_comment_subscription(
    &self,
    _user_id: i64,
    _payload: CreateCommentSubscriptionPayload,
  ) -> Result<CommentSubscriptionView, AppError> {
    todo!()
  }

  /// Delete a comment subscription for the given user.
  pub async fn delete_comment_subscription(
    &self,
    _user_id: i64,
    _id: i64,
  ) -> Result<(), AppError> {
    todo!()
  }
}
