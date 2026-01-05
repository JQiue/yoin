use tracing::level_filters::LevelFilter;
use tracing_subscriber::{EnvFilter, filter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::db::migrate;

mod config;
mod db;

#[tokio::main]
async fn main() {
  init_log();
  let config = config::Config::from_env().unwrap();
  let conn = migrate(&config.database_url).await.unwrap();
  conn.ping().await.unwrap();
}

fn init_log() {
  let target_filter = filter::Targets::new()
    .with_default(LevelFilter::TRACE)
    .with_target("sqlx::query", LevelFilter::OFF)
    .with_target("html5ever", LevelFilter::OFF)
    .with_target("rustls", LevelFilter::OFF);
  let env_filter = EnvFilter::try_from_default_env()
    .or_else(|_| EnvFilter::try_new("info"))
    .unwrap();
  tracing_subscriber::registry()
    .with(tracing_subscriber::fmt::layer().with_timer(fmt::time::LocalTime::rfc_3339()))
    .with(target_filter)
    .with(env_filter)
    .init();
}
