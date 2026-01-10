use axum::{
  Router,
  middleware::from_extractor_with_state,
  routing::{get, patch, post},
};
use sea_orm::DatabaseConnection;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
  config::Config,
  db::migrate,
  handler::{get_my_profile, health_check, login, register, update_my_profile},
  middleware::RequireAuth,
};

mod config;
mod db;
mod entity;
mod error;
mod handler;
mod helper;
mod middleware;
mod repo;
mod response;
mod service;

#[derive(Clone)]
struct AppState {
  conn: DatabaseConnection,
  jwt_key: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
  init_tracing();
  let config = Config::from_env()?;
  let conn = migrate(&config.database_url).await?;
  let addr = format!("{}:{}", config.host, config.port);
  let state = AppState {
    conn,
    jwt_key: config.jwt_key,
  };
  info!("🚀 Server running on http://{}", addr);
  axum::serve(TcpListener::bind(addr).await?, create_router(state)).await?;
  Ok(())
}

fn init_tracing() {
  tracing_subscriber::registry()
    .with(
      tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("debug,sqlx::query=off,hyper=debug,axum=debug")),
    )
    .with(tracing_subscriber::fmt::layer().with_target(false))
    .init();
}

fn create_router(state: AppState) -> Router {
  let public_routes = Router::new()
    .route("/health", get(health_check))
    .route("/auth/register", post(register))
    .route("/auth/login", post(login));

  let private_routes = Router::new()
    .route("/users/me", get(get_my_profile))
    .route("/users/me", patch(update_my_profile))
    .route_layer(from_extractor_with_state::<RequireAuth, AppState>(
      state.clone(),
    ));

  let api_routes = Router::new().merge(public_routes).merge(private_routes);

  Router::new()
    .nest("/api", api_routes)
    .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
    .with_state(state)
}
