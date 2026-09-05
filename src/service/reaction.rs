use std::collections::BTreeMap;

use migration::enums::{CommentStatus, ReactionActorType, ReactionTargetType};

use super::AppService;
use crate::{
  constants::reaction::ALLOWED_TYPES,
  entity::reactions,
  error::{AppError, ToAppError},
  handler::{
    comment::CommentView,
    reaction::{ListReactionsQuery, ReactionSummaryView, UpsertReactionPayload},
  },
  rbac::permissions::codes::SITE_MANAGE,
  repository::ReactionCreateData,
};

impl AppService {
  fn parse_target_type(value: &str) -> Result<ReactionTargetType, AppError> {
    match value {
      "comment" => Ok(ReactionTargetType::Comment),
      "page" => Ok(ReactionTargetType::Page),
      _ => Err(AppError::bad_request("invalid reaction target".to_string())),
    }
  }

  fn target_key(
    target_type: &ReactionTargetType,
    site_id: i64,
    comment_id: Option<i64>,
    page_path: &str,
  ) -> Result<String, AppError> {
    match target_type {
      ReactionTargetType::Comment => {
        let comment_id = comment_id.ok_or_else(|| {
          AppError::bad_request("comment_id is required for comment reactions".to_string())
        })?;
        Ok(format!("comment:{comment_id}"))
      }
      ReactionTargetType::Page => {
        if page_path.is_empty() {
          return Err(AppError::bad_request(
            "page_path is required for page reactions".to_string(),
          ));
        }
        Ok(format!("page:{site_id}:{page_path}"))
      }
    }
  }

  fn actor(
    user_id: Option<i64>,
    guest_id: Option<&str>,
  ) -> Result<(ReactionActorType, String), AppError> {
    match user_id {
      Some(id) => Ok((ReactionActorType::User, id.to_string())),
      None => match guest_id {
        Some(id) if !id.is_empty() => Ok((ReactionActorType::Guest, id.to_string())),
        _ => Err(AppError::bad_request("guest id is required".to_string())),
      },
    }
  }

  async fn ensure_reaction_target_visible(
    &self,
    user_id: Option<i64>,
    site_id: i64,
    target_type: &ReactionTargetType,
    comment_id: Option<i64>,
    page_path: &str,
  ) -> Result<String, AppError> {
    self
      .repo
      .site()
      .find_by_id(site_id)
      .await
      .with_op("find site by id")?
      .ok_or_else(|| AppError::site_not_found("Site not found".to_string()))?;

    if *target_type == ReactionTargetType::Comment {
      let comment_id = comment_id.ok_or_else(|| {
        AppError::bad_request("comment_id is required for comment reactions".to_string())
      })?;
      let comment = self
        .repo
        .comment()
        .find_by_id(comment_id)
        .await
        .with_op("find comment by id")?
        .ok_or_else(|| AppError::comment_not_found("Comment not found".to_string()))?;

      if comment.site_id != site_id || comment.page_path != page_path {
        return Err(AppError::comment_not_found("Comment not found".to_string()));
      }
      if !matches!(comment.status, CommentStatus::Approved) {
        return Err(AppError::comment_not_found("Comment not found".to_string()));
      }
      if comment.is_private {
        let can_see = match user_id {
          Some(id) => self.has_site_permission(id, SITE_MANAGE, site_id).await?,
          None => false,
        };
        if !can_see {
          return Err(AppError::comment_not_found("Comment not found".to_string()));
        }
      }
    }

    Self::target_key(target_type, site_id, comment_id, page_path)
  }

  fn summarize(
    reactions: impl IntoIterator<Item = reactions::Model>,
    actor_id: Option<&str>,
  ) -> ReactionSummaryView {
    let mut counts = BTreeMap::new();
    let mut my_reaction = None;
    for reaction in reactions {
      *counts.entry(reaction.r#type.clone()).or_insert(0) += 1;
      if actor_id == Some(reaction.actor_id.as_str()) {
        my_reaction = Some(reaction.r#type);
      }
    }
    ReactionSummaryView {
      counts,
      my_reaction,
    }
  }

  pub(crate) async fn attach_comment_reactions(
    &self,
    comments: &mut [CommentView],
    actor_id: Option<&str>,
  ) -> Result<(), AppError> {
    fn collect_ids(comments: &[CommentView], ids: &mut Vec<i64>) {
      for comment in comments {
        ids.push(comment.id);
        if let Some(replies) = &comment.replies {
          collect_ids(replies, ids);
        }
      }
    }

    let mut ids = Vec::new();
    collect_ids(comments, &mut ids);
    if ids.is_empty() {
      return Ok(());
    }
    let keys: Vec<String> = ids.iter().map(|id| format!("comment:{id}")).collect();
    let reactions = self
      .repo
      .reaction()
      .find_all_by_target_keys(&keys)
      .await
      .with_op("list reactions by comment ids")?;
    let mut grouped: BTreeMap<String, Vec<reactions::Model>> = BTreeMap::new();
    for reaction in reactions {
      grouped
        .entry(reaction.target_key.clone())
        .or_default()
        .push(reaction);
    }

    fn apply(
      comments: &mut [CommentView],
      grouped: &BTreeMap<String, Vec<reactions::Model>>,
      actor_id: Option<&str>,
    ) {
      for comment in comments {
        let key = format!("comment:{}", comment.id);
        comment.reactions =
          AppService::summarize(grouped.get(&key).cloned().unwrap_or_default(), actor_id);
        if let Some(replies) = &mut comment.replies {
          apply(replies, grouped, actor_id);
        }
      }
    }
    apply(comments, &grouped, actor_id);
    Ok(())
  }

  pub async fn upsert_reaction(
    &self,
    user_id: Option<i64>,
    guest_id: Option<&str>,
    payload: UpsertReactionPayload,
  ) -> Result<ReactionSummaryView, AppError> {
    if !ALLOWED_TYPES.contains(&payload.reaction.as_str()) {
      return Err(AppError::bad_request("invalid reaction".to_string()));
    }

    let target_type = Self::parse_target_type(&payload.target_type)?;
    let target_key = self
      .ensure_reaction_target_visible(
        user_id,
        payload.site_id,
        &target_type,
        payload.comment_id,
        &payload.page_path,
      )
      .await?;
    let (actor_type, actor_id) = Self::actor(user_id, guest_id)?;

    let existing = self
      .repo
      .reaction()
      .find_by_actor_and_target(actor_type.clone(), &actor_id, &target_key)
      .await
      .with_op("find reaction by actor and target")?;

    match existing {
      Some(existing) if existing.r#type == payload.reaction => {
        self
          .repo
          .reaction()
          .delete_by_id(existing.id)
          .await
          .with_op("delete reaction")?;
      }
      Some(existing) => {
        self
          .repo
          .reaction()
          .update_type(existing.id, payload.reaction)
          .await
          .with_op("update reaction")?;
      }
      None => {
        self
          .repo
          .reaction()
          .create(ReactionCreateData {
            site_id: payload.site_id,
            target_type,
            comment_id: payload.comment_id,
            page_path: payload.page_path,
            actor_type,
            actor_id: actor_id.clone(),
            r#type: payload.reaction,
            target_key: target_key.clone(),
          })
          .await
          .with_op("insert reaction")?;
      }
    }

    let reactions = self
      .repo
      .reaction()
      .find_all_by_target_key(&target_key)
      .await
      .with_op("list reactions by target")?;
    Ok(Self::summarize(reactions, Some(&actor_id)))
  }

  pub async fn list_reactions(
    &self,
    user_id: Option<i64>,
    guest_id: Option<&str>,
    qs: ListReactionsQuery,
  ) -> Result<ReactionSummaryView, AppError> {
    let target_type = Self::parse_target_type(&qs.target_type)?;
    let target_key = self
      .ensure_reaction_target_visible(
        user_id,
        qs.site_id,
        &target_type,
        qs.comment_id,
        &qs.page_path,
      )
      .await?;
    let actor_id = Self::actor(user_id, guest_id).ok().map(|(_, id)| id);
    let reactions = self
      .repo
      .reaction()
      .find_all_by_target_key(&target_key)
      .await
      .with_op("list reactions by target")?;
    Ok(Self::summarize(reactions, actor_id.as_deref()))
  }
}
