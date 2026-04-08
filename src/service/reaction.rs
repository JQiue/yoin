use super::AppService;
use crate::{
  error::AppError,
  handler::reaction::{CreateReactionPayload, ReactionView},
};

impl AppService {
  /// Create a reaction (like/upvote) for a user.
  ///
  /// This is currently unimplemented (`todo!()`).
  pub async fn create_reaction(
    &self,
    _user_id: i64,
    _payload: CreateReactionPayload,
  ) -> Result<ReactionView, AppError> {
    todo!()
  }

  pub async fn delete_reaction(&self, _user_id: i64, _id: i64) -> Result<(), AppError> {
    // Delete a reaction for a user. (Unimplemented: `todo!()`.)
    todo!()
  }
}
