use helpers::uuid::{Alphabet, nanoid};
use serde::Deserialize;
use tracing::warn;

fn default_database_url() -> String {
  "sqlite://./yoin.sqlite?mode=rwc".to_string()
}

fn default_host() -> String {
  "127.0.0.1".to_string()
}

fn default_port() -> u16 {
  7410
}

fn default_jwt_key() -> String {
  warn!("JWT_KEY is not set; using ephemeral key. All tokens will be invalid after restart.");
  nanoid(&Alphabet::DEFAULT, 32)
}

#[derive(Deserialize)]
pub struct Config {
  #[serde(default = "default_database_url")]
  pub database_url: String,
  #[serde(default = "default_host")]
  pub host: String,
  #[serde(default = "default_port")]
  pub port: u16,
  #[serde(default = "default_jwt_key")]
  pub jwt_key: String,
}

impl Config {
  pub fn from_env() -> Result<Config, envy::Error> {
    let _ = dotenvy::dotenv_override();
    let config: Config = envy::from_env()?;

    if config.jwt_key.trim().len() < 32 {
      return Err(envy::Error::Custom(
        "JWT_KEY must be at least 32 characters".to_string(),
      ));
    }

    Ok(config)
  }
}
