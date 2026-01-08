use axum::{Json, Router, response::IntoResponse, routing::get};
use serde_json::json;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
  config::Config,
  db::migrate,
  error::{AppError, ToAppError},
  response::ApiResponse,
};

mod config;
mod db;
mod entity;
mod error;
mod response;

#[derive(Clone)]
struct AppState {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
  init_log();
  let config = Config::from_env()?;
  let conn = migrate(&config.database_url).await?;
  let addr = format!("{}:{}", config.host, config.port);
  let state = AppState {};
  info!("listening on http://{}", addr);
  axum::serve(TcpListener::bind(addr).await?, router(state)).await?;
  Ok(())
}

fn init_log() {
  tracing_subscriber::registry()
    .with(
      tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("debug,sqlx::query=off,hyper=debug,axum=debug")),
    )
    .with(tracing_subscriber::fmt::layer().with_target(false))
    .init();
}

fn router(state: AppState) -> Router {
  Router::new()
    .nest("/api/v1", Router::new().route("/health", get(health_check)))
    .layer(TraceLayer::new_for_http())
    .with_state(state)
}

async fn health_check() -> Result<ApiResponse<()>, AppError> {
  Ok(ApiResponse::success(()))
}
