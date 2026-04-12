use super::AppService;
use crate::{
  error::AppError,
  handler::auth::{CreateOauthProviderPayload, ExternalExchangePayload, UserWithToken},
};

impl AppService {
  /// Create an OAuth provider configuration (currently a placeholder).
  pub async fn create_oauth_provider(
    &self,
    _payload: CreateOauthProviderPayload,
  ) -> Result<(), AppError> {
    Ok(())
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
