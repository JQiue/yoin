use crate::error::AppError;

#[derive(Debug, Clone, Copy)]
pub enum ModerationDecision {
  Allow,
  Review,
  Reject,
}

#[derive(Debug)]
pub struct ModerationResult {
  pub decision: ModerationDecision,
  pub provider: String,
  pub reason: Option<String>,
  pub score: Option<f32>,
}

#[derive(Debug, Clone)]
pub struct ModerationInput {
  pub content: String,
  pub email: String,
}

pub(crate) trait CommentModerator {
  async fn check(&self, input: ModerationInput) -> Result<ModerationResult, AppError>;
}
