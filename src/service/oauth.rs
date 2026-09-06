use helpers::{
  hash::argon2,
  jwt,
  time::utc_now,
  uuid::{Alphabet, nanoid},
};

use super::AppService;
use crate::{
  error::{AppError, ToAppError},
  handler::auth::{
    CreateOauthProviderPayload, ExternalExchangePayload, OauthProviderView,
    PublicOauthProviderView, UpdateOauthProviderPayload, UserProfile, UserWithToken,
  },
  helper::{OauthProfile, OauthService, generate_avatar},
  rbac::bootstrap::ensure_super_admin_binding,
  repository::{
    OauthProviderCreateData, OauthProviderUpdateData, UserCreateData, UserIdentityCreateData,
  },
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

  pub async fn list_public_oauth_providers(
    &self,
  ) -> Result<Vec<PublicOauthProviderView>, AppError> {
    let providers = self
      .repo
      .oauth_provider()
      .find_enabled_public()
      .await
      .with_op("find public oauth providers")?;
    Ok(
      providers
        .into_iter()
        .map(|provider| PublicOauthProviderView {
          provider_code: provider.provider_code,
        })
        .collect(),
    )
  }

  pub async fn login_or_register_oauth_user(
    &self,
    provider: &str,
    profile: OauthProfile,
    jwt_key: &str,
  ) -> Result<UserWithToken, AppError> {
    let provider = provider.trim().to_lowercase();
    OauthService::new(&provider)?;
    let datetime = utc_now().naive_utc();
    let email = profile
      .email
      .as_deref()
      .map(str::trim)
      .filter(|value| !value.is_empty())
      .map(str::to_string);
    let account_email = email
      .clone()
      .unwrap_or_else(|| format!("{provider}+{}@oauth.yoin.local", profile.id));

    if let Some(identity) = self
      .repo
      .user_identity()
      .find_by_provider_subject(&provider, &profile.id)
      .await
      .with_op("find oauth identity")?
    {
      let user = self
        .repo
        .user()
        .find_by_id(identity.user_id)
        .await
        .with_op("find user by id")?
        .ok_or(AppError::user_not_found("User not found".to_string()))?;
      return Self::signed_user(user, jwt_key);
    }

    let existing_user = if let Some(email) = email.as_ref() {
      self
        .repo
        .user()
        .find_by_email(email)
        .await
        .with_op("find user by email")?
    } else {
      None
    };

    let user = if let Some(user) = existing_user {
      user
    } else {
      let is_first_user = self
        .repo
        .user()
        .exists_any()
        .await
        .with_op("is first user")?;
      if is_first_user {
        self
          .repo
          .site()
          .create_default(datetime)
          .await
          .with_op("create default site")?;
      }
      let password = argon2(
        &nanoid(&Alphabet::DEFAULT, 32),
        &nanoid(&Alphabet::DEFAULT, 8),
      )
      .with_op("hash_password")?;
      let nickname = if profile.nickname.trim().is_empty() {
        provider.clone()
      } else {
        profile.nickname.clone()
      };
      let avatar = if profile.avatar.trim().is_empty() {
        generate_avatar(&account_email)
      } else {
        profile.avatar.clone()
      };
      let new_user = self
        .repo
        .user()
        .create(UserCreateData {
          nickname,
          password,
          email: account_email,
          website: String::new(),
          avatar,
          datetime,
        })
        .await
        .with_op("insert_user")?;
      if is_first_user {
        ensure_super_admin_binding(&self.repo, new_user.id).await?;
      }
      new_user
    };

    self
      .repo
      .user_identity()
      .create(UserIdentityCreateData {
        user_id: user.id,
        provider,
        provider_user_id: profile.id,
        email,
        profile: Some(serde_json::json!({
          "nickname": profile.nickname,
          "avatar": profile.avatar,
        })),
        datetime,
      })
      .await
      .with_op("insert oauth identity")?;

    Self::signed_user(user, jwt_key)
  }

  fn signed_user(
    user: crate::entity::users::Model,
    jwt_key: &str,
  ) -> Result<UserWithToken, AppError> {
    let token = jwt::sign(user.id, jwt_key, 30 * 24 * 60 * 60).with_op("sign jwt token")?;
    Ok(UserWithToken {
      token,
      user: UserProfile {
        avatar: user.avatar,
        nickname: user.nickname,
        website: user.website,
        email: user.email,
      },
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
