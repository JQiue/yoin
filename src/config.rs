use serde::Deserialize;

fn default_database_url() -> String {
  "sqlite://./yoin.sqlite?mode=rwc".to_string()
}

fn default_host() -> String {
  "127.0.0.1".to_string()
}

fn default_port() -> u16 {
  7410
}

#[derive(Deserialize)]
pub struct Config {
  #[serde(default = "default_database_url")]
  pub database_url: String,
  #[serde(default = "default_host")]
  pub host: String,
  #[serde(default = "default_port")]
  pub port: u16,
  pub jwt_key: String,
}

impl Config {
  pub fn from_env() -> Result<Config, envy::Error> {
    dotenvy::dotenv_override().ok();
    envy::from_env()
  }
}
