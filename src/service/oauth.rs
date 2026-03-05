use super::AppService;
use crate::{error::AppError, handler::auth::CreateOauthProviderPayload};

impl AppService {
  pub async fn create_oauth_provider(
    &self,
    _payload: CreateOauthProviderPayload,
  ) -> Result<(), AppError> {
    Ok(())
  }
}
