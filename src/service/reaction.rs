use super::AppService;
use crate::{
  error::AppError,
  handler::reaction::{CreateReactionPayload, ReactionView},
};

impl AppService {
  pub async fn create_reaction(
    &self,
    _user_id: i64,
    _payload: CreateReactionPayload,
  ) -> Result<ReactionView, AppError> {
    todo!()
  }

  pub async fn delete_reaction(&self, _user_id: i64, _id: i64) -> Result<(), AppError> {
    todo!()
  }
}
