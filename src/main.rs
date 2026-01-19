use std::{
  collections::{HashMap, HashSet},
  net::SocketAddr,
  sync::{Arc, Mutex},
};

use axum::{
  Router,
  http::StatusCode,
  middleware::{self, from_extractor_with_state},
  response::{IntoResponse, Response},
  routing::{get, post},
};
use migration::{
  enums::UserRole,
  prelude::{DateTime, Utc},
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
  entity::{
    prelude::{Sites, Users},
    sites::SiteConfig,
  },
  error::AppError,
  extractor::{OptionnalAuth, RemoteIp, RequireAuth},
  handler::{auth, comment, health, site, user},
  repo::SiteRepo,
};

mod config;
mod db;
mod entity;
mod error;
pub mod extractor;
mod handler;
mod helper;
mod repo;
mod response;
mod service;

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
enum UserKey {
  UserId(i64),
  Ip(String),
}

#[derive(Clone)]
struct AppState {
  conn: DatabaseConnection,
  jwt_key: String,
  site_config: Arc<Mutex<HashMap<i64, SiteConfig>>>,
  admin_ids: Arc<Mutex<HashSet<i64>>>,
  rate_limit_cache: Arc<Mutex<HashMap<(i64, UserKey), DateTime<Utc>>>>,
}

impl AppState {
  fn is_admin(&self, user_id: i64) -> bool {
    self.admin_ids.lock().unwrap().contains(&user_id)
  }

  fn check_rate_limit(
    &self,
    site_id: i64,
    user_id: Option<i64>,
    remote_ip: String,
    limit_seconds: i64,
  ) -> Result<(), AppError> {
    let user_key = match user_id {
      Some(id) => UserKey::UserId(id),
      None => UserKey::Ip(remote_ip),
    };
    let key = (site_id, user_key);
    let mut cache = self.rate_limit_cache.lock().unwrap();
    let now = Utc::now();

    if let Some(last_comment_time) = cache.get(&key) {
      let elapsed = now.signed_duration_since(*last_comment_time).num_seconds();
      if elapsed < limit_seconds {
        return Err(AppError::bad_request(format!(
          "The comment is too fast. Please try again in {} seconds",
          limit_seconds - elapsed
        )));
      }
    }
    cache.insert(key, now);
    Ok(())
  }

  async fn preload_configs(&self) {
    let sites = Sites::find_all(&self.conn).await.unwrap();
    for site in sites {
      self
        .site_config
        .lock()
        .unwrap()
        .insert(site.id, site.config);
    }

    let users = Users::find().all(&self.conn).await.unwrap();
    for user in users {
      if user.role == UserRole::Admin {
        self.admin_ids.lock().unwrap().insert(user.id);
      }
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
    admin_ids: Arc::new(Mutex::new(HashSet::new())),
    rate_limit_cache: Arc::new(Mutex::new(HashMap::new())),
  };
  state.preload_configs().await;
  info!("🚀 Server running on http://{}", addr);
  axum::serve(
    TcpListener::bind(addr).await?,
    create_router(state).into_make_service_with_connect_info::<SocketAddr>(),
  )
  .await?;
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
    .route("/health", get(health::health_check))
    .route("/auth/register", post(auth::register))
    .route("/auth/login", post(auth::login))
    .route("/comments", post(comment::create))
    .route_layer(from_extractor_with_state::<OptionnalAuth, AppState>(
      state.clone(),
    ));

  let private_routes = Router::new()
    .route("/users/me", get(user::profile).patch(user::update_profile))
    .route(
      "/sites",
      post(site::create).get(site::list).patch(site::update),
    )
    .route_layer(from_extractor_with_state::<RequireAuth, AppState>(
      state.clone(),
    ));

  let api_routes = Router::new()
    .merge(public_routes)
    .merge(private_routes)
    .route_layer(from_extractor_with_state::<RemoteIp, AppState>(
      state.clone(),
    ));

  Router::new()
    .nest("/api", api_routes)
    .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
    .layer(middleware::map_response(common_error_interceptor))
    .with_state(state)
}

async fn common_error_interceptor(res: Response) -> Response {
  let status = res.status();
  if status == StatusCode::NOT_FOUND || status == StatusCode::METHOD_NOT_ALLOWED {
    let msg = if status == StatusCode::NOT_FOUND {
      "Not found"
    } else {
      "Method not allowed"
    };
    return AppError::bad_request(msg.to_string()).into_response();
  }
  res
}
