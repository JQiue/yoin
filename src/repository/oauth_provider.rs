use sea_orm::DatabaseConnection;

pub trait OauthProviderTrait {}

pub struct OauthProvider {
  pub conn: &'static DatabaseConnection,
}

impl OauthProvider {}
