use axum::{Json, Router, response::IntoResponse, routing::get};
use serde_json::json;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

use crate::{config::Config, db::migrate};

mod config;
mod db;
mod entity;

#[derive(Clone)]
struct AppState {}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
  init_log();
  let config = Config::from_env().unwrap();
  let conn = migrate(&config.database_url).await.unwrap();
  let addr = format!("{}:{}", config.host, config.port);
  let state = AppState {};
  info!("listening on http://{}", addr);
  app(&addr, router(state)).await;
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

async fn app(addr: &str, router: Router) {
  axum::serve(TcpListener::bind(addr).await.unwrap(), router)
    .await
    .unwrap();
}

fn router(state: AppState) -> Router {
  Router::new()
    .nest("/api/v1", Router::new().route("/health", get(health_check)))
    .layer(TraceLayer::new_for_http())
    .with_state(state)
}

async fn health_check() -> impl IntoResponse {
  Json(json!({
    "status": "ok"
  }))
}
