use helpers::time::utc_now;

use super::AppService;
use crate::{
  error::{AppError, ToAppError},
  handler::auth::{
    CreateOauthProviderPayload, ExternalExchangePayload, OauthProviderView,
    UpdateOauthProviderPayload, UserWithToken,
  },
  helper::OauthService,
  repository::{OauthProviderCreateData, OauthProviderUpdateData},
};

pub struct OauthProviderRuntimeConfig {
  pub client_id: String,
  pub client_secret: String,
  pub redirect_uri: String,
}

impl AppService {
  pub async fn list_oauth_providers(&self) -> Result<Vec<OauthProviderView>, AppError> {
    let providers = self
      .repo
      .oauth_provider()
      .find_all()
      .await
      .with_op("find all oauth providers")?;
    Ok(
      providers
        .into_iter()
        .map(OauthProviderView::from_model)
        .collect(),
    )
  }

  pub async fn create_oauth_provider(
    &self,
    payload: CreateOauthProviderPayload,
  ) -> Result<OauthProviderView, AppError> {
    let provider_code = payload.provider_code.trim().to_lowercase();
    OauthService::new(&provider_code)?;
    if payload.client_id.trim().is_empty()
      || payload.client_secret.trim().is_empty()
      || payload.redirect_uri.trim().is_empty()
    {
      return Err(AppError::bad_request(
        "client_id, client_secret and redirect_uri are required".to_string(),
      ));
    }
    if let Some(site_id) = payload.site_id {
      self
        .repo
        .site()
        .find_by_id(site_id)
        .await
        .with_op("find site for oauth provider")?
        .ok_or_else(|| AppError::site_not_found("site not found".to_string()))?;
    }

    let existing = self
      .repo
      .oauth_provider()
      .find_by_site_and_code(payload.site_id, &provider_code)
      .await
      .with_op("find oauth provider by site and code")?;
    if existing.is_some() {
      return Err(AppError::bad_request(
        "oauth provider already exists for this scope".to_string(),
      ));
    }

    let provider = self
      .repo
      .oauth_provider()
      .create(OauthProviderCreateData {
        site_id: payload.site_id,
        provider_code,
        enabled: payload.enabled.unwrap_or(true),
        client_id: payload.client_id.trim().to_string(),
        client_secret: payload.client_secret.trim().to_string(),
        redirect_uri: payload.redirect_uri.trim().to_string(),
        datetime: utc_now().naive_local(),
      })
      .await
      .with_op("insert oauth provider")?;
    Ok(OauthProviderView::from_model(provider))
  }

  pub async fn update_oauth_provider(
    &self,
    id: i64,
    payload: UpdateOauthProviderPayload,
  ) -> Result<OauthProviderView, AppError> {
    self
      .repo
      .oauth_provider()
      .find_by_id(id)
      .await
      .with_op("find oauth provider")?
      .ok_or_else(|| AppError::invalid_oauth_provider("oauth provider not found".to_string()))?;

    if let Some(Some(site_id)) = payload.site_id {
      self
        .repo
        .site()
        .find_by_id(site_id)
        .await
        .with_op("find site for oauth provider")?
        .ok_or_else(|| AppError::site_not_found("site not found".to_string()))?;
    }

    let provider = self
      .repo
      .oauth_provider()
      .update(OauthProviderUpdateData {
        id,
        site_id: payload.site_id,
        provider_code: None,
        enabled: payload.enabled,
        client_id: payload.client_id.map(|value| value.trim().to_string()),
        client_secret: payload.client_secret.map(|value| value.trim().to_string()),
        redirect_uri: payload.redirect_uri.map(|value| value.trim().to_string()),
        datetime: utc_now().naive_local(),
      })
      .await
      .with_op("update oauth provider")?;
    Ok(OauthProviderView::from_model(provider))
  }

  pub async fn get_oauth_provider_config(
    &self,
    provider_code: &str,
  ) -> Result<OauthProviderRuntimeConfig, AppError> {
    OauthService::new(provider_code)?;
    let provider = self
      .repo
      .oauth_provider()
      .find_enabled_by_site_and_code(None, provider_code)
      .await
      .with_op("find enabled oauth provider")?
      .ok_or_else(|| {
        AppError::invalid_oauth_provider(format!("{provider_code} provider config not set"))
      })?;
    Ok(OauthProviderRuntimeConfig {
      client_id: provider.client_id,
      client_secret: provider.client_secret,
      redirect_uri: provider.redirect_uri,
    })
  }

  /// Exchange external OAuth credentials for a local user and JWT (TODO).
  pub async fn external_auth_exchange(
    &self,
    _payload: ExternalExchangePayload,
    _jwt_key: &str,
  ) -> Result<UserWithToken, AppError> {
    todo!()
  }
}
