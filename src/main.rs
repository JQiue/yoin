use std::{
  collections::HashMap,
  sync::{Arc, Mutex},
};

use axum::{
  Router,
  middleware::from_extractor_with_state,
  routing::{get, post},
};
use sea_orm::{DatabaseConnection, EntityTrait};
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
  config::Config,
  db::migrate,
  entity::{prelude::Sites, sites::SiteConfig},
  handler::{
    create_comment, create_site, get_my_profile, health_check, list_sites, login, register,
    update_my_profile,
  },
  middleware::{OptionnalAuth, RequireAuth},
  repo::SiteRepo,
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
  site_config: Arc<Mutex<HashMap<i64, SiteConfig>>>,
}

impl AppState {
  async fn preload_site_configs(&self) {
    let sites = Sites::get_sites(&self.conn).await.unwrap();
    for site in sites {
      self
        .site_config
        .lock()
        .unwrap()
        .insert(site.id, site.config);
    }
  }

  async fn get_site_config(&self, site_id: i64) -> Option<SiteConfig> {
    self.site_config.lock().unwrap().get(&site_id).cloned()
  }
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
    site_config: Arc::new(Mutex::new(HashMap::new())),
  };
  state.preload_site_configs().await;
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
    .route("/auth/login", post(login))
    .route("/comments", post(create_comment))
    .route_layer(from_extractor_with_state::<OptionnalAuth, AppState>(
      state.clone(),
    ));

  let private_routes = Router::new()
    .route("/users/me", get(get_my_profile).patch(update_my_profile))
    .route(
      "/sites",
      post(create_site).get(list_sites), // .patch(update_site_config),
    )
    .route_layer(from_extractor_with_state::<RequireAuth, AppState>(
      state.clone(),
    ));

  let api_routes = Router::new().merge(public_routes).merge(private_routes);

  Router::new()
    .nest("/api", api_routes)
    .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
    .with_state(state)
}
