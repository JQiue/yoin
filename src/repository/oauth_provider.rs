use sea_orm::DatabaseConnection;

pub struct OauthProvider {
  pub conn: &'static DatabaseConnection,
}

impl OauthProvider {}
