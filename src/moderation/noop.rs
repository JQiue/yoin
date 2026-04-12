use crate::error::AppError;

use super::types::{CommentModerator, ModerationDecision, ModerationInput, ModerationResult};

pub struct NoopModerator;

impl CommentModerator for NoopModerator {
  async fn check(&self, _input: ModerationInput) -> Result<ModerationResult, AppError> {
    Ok(ModerationResult {
      decision: ModerationDecision::Allow,
      provider: "noop".to_string(),
      reason: None,
      score: None,
    })
  }
}
