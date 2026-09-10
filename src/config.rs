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
  /// `YOIN_MIGRATE`：设 0/false/off/no 时启动不执行迁移（envy 会把字段名大写后与环境变量匹配）
  #[serde(default)]
  pub yoin_migrate: Option<String>,
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

  /// 启动时是否执行迁移，默认执行。
  ///
  /// 容器 / 无服务器平台（例如 Vercel）建议设 `YOIN_MIGRATE=0`，改在部署时单独跑
  /// `yoin migrate`：那里进程随时可能被回收，每次冷启动都跑迁移既慢又可能互相竞争。
  pub fn migrate_on_start(&self) -> bool {
    !matches!(
      self.yoin_migrate.as_deref().map(str::trim),
      Some("0" | "false" | "off" | "no")
    )
  }
}
