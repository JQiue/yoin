use std::collections::BTreeSet;

use helpers::time::utc_now;
use migration::enums::UserRoleBindingScopeType;

use super::AppService;
use crate::{
  constants::reaction,
  entity::sites::SiteConfig,
  error::{AppError, ToAppError},
  handler::site::{CreateSitePayload, PublicSiteConfigView, SiteView, UpdateSitePayload},
  rbac::permissions::codes::SITE_MANAGE,
  repository::{SiteCreateData, SiteUpdateData},
};

impl AppService {
  fn normalize_site_config(config: SiteConfig) -> Result<SiteConfig, AppError> {
    let mut seen = BTreeSet::new();
    let mut allowed_reactions = Vec::with_capacity(config.allowed_reactions.len());
    for reaction_type in config.allowed_reactions {
      if !reaction::is_globally_allowed(&reaction_type) {
        return Err(AppError::bad_request(format!(
          "invalid reaction: {reaction_type}"
        )));
      }
      if seen.insert(reaction_type.clone()) {
        allowed_reactions.push(reaction_type);
      }
    }
    Ok(SiteConfig {
      allowed_reactions,
      ..config
    })
  }

  /// Public site config used by the embeddable widget.
  pub async fn get_public_site_config(
    &self,
    site_id: i64,
  ) -> Result<PublicSiteConfigView, AppError> {
    let site = self
      .repo
      .site()
      .find_by_id(site_id)
      .await
      .with_op("find site by id")?
      .ok_or(AppError::site_not_found("Site not found".to_string()))?;
    Ok(PublicSiteConfigView {
      allow_anonymous: site.config.allow_anonymous,
      allow_private: site.config.allow_private,
      max_comment_length: site.config.max_comment_length,
      comment_limit_seconds: site.config.comment_limit_seconds,
      allowed_reactions: site.config.allowed_reactions,
    })
  }

  /// List all sites.
  ///
  /// Requires the user to have global `SITE_MANAGE` permission.
  pub async fn list_sites(&self, user_id: i64) -> Result<Vec<SiteView>, AppError> {
    self
      .require_permission(user_id, SITE_MANAGE, UserRoleBindingScopeType::Global, None)
      .await?;
    let sites = self
      .repo
      .site()
      .find_all()
      .await
      .with_op("find all sites")?
      .iter()
      .map(|site| SiteView {
        id: site.id,
        name: site.name.clone(),
        config: site.config.clone(),
        url: site.url.clone(),
      })
      .collect();
    Ok(sites)
  }

  /// Create a new site.
  pub async fn create_site(
    &self,
    user_id: i64,
    payload: CreateSitePayload,
  ) -> Result<SiteView, AppError> {
    self
      .require_permission(user_id, SITE_MANAGE, UserRoleBindingScopeType::Global, None)
      .await?;
    let datetime = utc_now().naive_utc();
    let config = Self::normalize_site_config(payload.config)?;
    let site = self
      .repo
      .site()
      .create(SiteCreateData {
        name: payload.name,
        url: payload.url,
        config,
        datetime,
      })
      .await
      .with_op("insert_site")?;
    Ok(SiteView {
      id: site.id,
      name: site.name,
      config: site.config,
      url: site.url,
    })
  }

  /// Update an existing site.
  pub async fn update_site(
    &self,
    user_id: i64,
    payload: UpdateSitePayload,
  ) -> Result<SiteView, AppError> {
    self
      .require_permission(user_id, SITE_MANAGE, UserRoleBindingScopeType::Global, None)
      .await?;
    let site = self
      .repo
      .site()
      .find_by_id(payload.id)
      .await
      .with_op("find site by id")?
      .ok_or(AppError::site_not_found("Site not found".to_string()))?;
    let config = payload
      .config
      .map(Self::normalize_site_config)
      .transpose()?;
    let site = self
      .repo
      .site()
      .update(SiteUpdateData {
        id: site.id,
        name: payload.name,
        url: payload.url,
        config,
        datetime: utc_now().naive_utc(),
      })
      .await
      .with_op("update site")?;

    Ok(SiteView {
      id: site.id,
      name: site.name,
      config: site.config,
      url: site.url,
    })
  }
}
