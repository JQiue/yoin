use helpers::time::utc_now;
use migration::enums::UserRoleBindingScopeType;

use super::AppService;
use crate::{
  error::{AppError, ToAppError},
  handler::site::{CreateSitePayload, SiteView, UpdateSitePayload},
  rbac::permissions::codes::SITE_MANAGE,
  repository::{SiteCreateData, SiteUpdateData},
};

impl AppService {
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
    let site = self
      .repo
      .site()
      .create(SiteCreateData {
        name: payload.name,
        url: payload.url,
        config: payload.config,
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

    let site = self
      .repo
      .site()
      .update(SiteUpdateData {
        id: site.id,
        name: payload.name,
        url: payload.url,
        config: payload.config,
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
