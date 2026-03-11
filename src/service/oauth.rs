use super::AppService;
use crate::{
  error::AppError,
  handler::auth::{CreateOauthProviderPayload, ExternalExchangePayload, UserWithToken},
};

impl AppService {
  pub async fn create_oauth_provider(
    &self,
    _payload: CreateOauthProviderPayload,
  ) -> Result<(), AppError> {
    Ok(())
  }

  pub async fn external_auth_exchange(
    &self,
    _payload: ExternalExchangePayload,
    _jwt_key: &str,
  ) -> Result<UserWithToken, AppError> {
    todo!()
  }
}
