use helpers::time::utc_now;
use migration::enums::UserRole;

use super::AppService;
use crate::{
  error::{AppError, ToAppError},
  handler::site::{CreateSitePayload, SiteView, UpdateSitePayload},
  repository::{SiteCreateData, SiteRepositoryTrait, SiteUpdateData, UserRepositoryTrait},
};

impl AppService {
  pub async fn list_sites(&self, user_id: i64) -> Result<Vec<SiteView>, AppError> {
    let user = self
      .repo
      .user()
      .find_by_id(user_id)
      .await
      .with_op("find user by id")?
      .ok_or(AppError::user_not_found("User not found".to_string()))?;

    if user.role != UserRole::Admin {
      return Err(AppError::forbidden(
        "You are not admin".to_string() + &format!("{} {:?}", user.id, user.role),
      ));
    }

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

  pub async fn create_site(
    &self,
    user_id: i64,
    payload: CreateSitePayload,
  ) -> Result<SiteView, AppError> {
    let user = self
      .repo
      .user()
      .find_by_id(user_id)
      .await
      .with_op("find user by id")?
      .ok_or(AppError::user_not_found("User not found".to_string()))?;

    if user.role != UserRole::Admin {
      return Err(AppError::forbidden(
        "You are not admin".to_string() + &user.id.to_string(),
      ));
    }
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

  pub async fn update_site(
    &self,
    user_id: i64,
    payload: UpdateSitePayload,
  ) -> Result<SiteView, AppError> {
    let user = self
      .repo
      .user()
      .find_by_id(user_id)
      .await
      .with_op("find user by id")?
      .ok_or(AppError::user_not_found("User not found".to_string()))?;

    if user.role != UserRole::Admin {
      return Err(AppError::forbidden(
        "You are not admin".to_string() + &user.id.to_string(),
      ));
    }

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
