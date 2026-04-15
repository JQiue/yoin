//! Moderation subsystem.
//!
//! Provides different moderation providers (LLM based, noop, etc.) and related
//! types. The module re‑exports the primary `LLMModerator`, `NoopModerator`
//! and associated data structures for convenient use throughout the
//! application.

pub mod llm;
pub mod noop;
pub mod types;

pub use crate::constants::moderation::provider_kind;
pub use llm::LLMModerator;
pub use noop::NoopModerator;
pub use types::{ModerationDecision, ModerationInput, ModerationResult};
