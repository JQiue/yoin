use serde::Deserialize;

fn default_database_url() -> String {
  "sqlite://./yoin.sqlite?mode=rwc".to_string()
}

#[derive(Deserialize)]
pub struct Config {
  #[serde(default = "default_database_url")]
  pub database_url: String,
}

impl Config {
  pub fn from_env() -> Result<Config, envy::Error> {
    dotenvy::dotenv_override().ok();
    Ok(envy::from_env()?)
  }
}
