use async_openai::{
  Client,
  config::OpenAIConfig,
  types::chat::{
    ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
    CreateChatCompletionRequestArgs,
  },
};
use serde::Deserialize;

use crate::{
  error::{AppError, ToAppError},
  moderation::{ModerationDecision, ModerationInput, ModerationResult},
};

const DEFAULT_PROMPT: &str = r#"You are a comment moderation system.
Review the user comment and return only JSON.

Rules:
- decision must be one of: allow, review, reject
- reject for obvious spam, scams, malicious links, abusive or illegal content
- review for borderline, ambiguous, or uncertain content
- allow for normal comments
- score is a float between 0 and 1 where higher means more risky

Return exactly:
{"decision":"allow|review|reject","reason":"short reason","score":0.0}"#;

#[derive(Debug)]
pub struct LLMModerator {
  pub prompt: String,
  pub model: String,
  pub api_key: String,
  pub api_base: String,
}

#[derive(Debug, Deserialize)]
struct LlmModerationPayload {
  decision: String,
  reason: Option<String>,
  score: Option<f32>,
}

impl LLMModerator {
  pub fn new(model: String, api_key: String, api_base: String) -> Self {
    Self {
      prompt: DEFAULT_PROMPT.to_string(),
      model,
      api_key,
      api_base,
    }
  }

  pub fn with_prompt(model: String, api_key: String, prompt: String, api_base: String) -> Self {
    Self {
      prompt,
      model,
      api_key,
      api_base,
    }
  }

  fn map_decision(raw: &str) -> ModerationDecision {
    match raw.trim().to_ascii_lowercase().as_str() {
      "allow" => ModerationDecision::Allow,
      "reject" => ModerationDecision::Reject,
      "review" => ModerationDecision::Review,
      _ => ModerationDecision::Review,
    }
  }
}

impl super::types::CommentModerator for LLMModerator {
  async fn check(&self, input: ModerationInput) -> Result<ModerationResult, AppError> {
    let config = OpenAIConfig::new()
      .with_api_key(self.api_key.clone())
      .with_api_base(self.api_base.clone());
    let client = Client::with_config(config);
    let system_message = ChatCompletionRequestSystemMessageArgs::default()
      .content(self.prompt.clone())
      .build()
      .with_op("build llm moderation system message")?;
    let user_message = ChatCompletionRequestUserMessageArgs::default()
      .content(format!(
        "Moderate this comment.\nemail: {}\ncontent:\n{}",
        input.email, input.content
      ))
      .build()
      .with_op("build llm moderation user message")?;
    let request = CreateChatCompletionRequestArgs::default()
      .model(self.model.clone())
      .temperature(0.0)
      .max_tokens(128u16)
      .messages([system_message.into(), user_message.into()])
      .build()
      .with_op("build llm moderation request")?;
    let response = client
      .chat()
      .create(request)
      .await
      .with_op("call llm moderation")?;
    let content = response
      .choices
      .first()
      .and_then(|choice| choice.message.content.as_deref())
      .ok_or_else(|| AppError::Internal {
        msg: "LLM moderation returned empty content".to_string(),
        source: None,
      })?;
    let parsed: LlmModerationPayload =
      serde_json::from_str(content).with_op("parse llm moderation response")?;
    Ok(ModerationResult {
      decision: Self::map_decision(&parsed.decision),
      provider: "llm".to_string(),
      reason: parsed.reason,
      score: parsed.score,
    })
  }
}
