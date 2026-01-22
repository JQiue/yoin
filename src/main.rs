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

use std::{
  collections::{HashMap, HashSet},
  net::SocketAddr,
  sync::Arc,
};

use axum::{
  Router,
  body::Body,
  http::{HeaderMap, StatusCode, header},
  middleware::{self, from_extractor, from_extractor_with_state},
  response::{IntoResponse, Response},
  routing::{get, post},
};
use migration::{
  enums::UserRole,
  prelude::{DateTime, Utc},
};
use sea_orm::{DatabaseConnection, EntityTrait};
use tokio::{net::TcpListener, sync::Mutex};
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::info;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
  config::Config,
  db::migrate,
  entity::{
    prelude::{Sites, Users},
    sites::SiteConfig,
  },
  error::{AppError, ToAppError},
  extractor::{OptionnalAuth, RemoteIp, RequireAuth},
  handler::{auth, comment, health, site, user},
  repo::SiteRepo,
};

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
enum UserKey {
  UserId(i64),
  Ip(String),
}

struct AppState {
  conn: DatabaseConnection,
  jwt_key: String,
  site_config: Mutex<HashMap<i64, SiteConfig>>,
  admin_ids: Mutex<HashSet<i64>>,
  // todo There is no expiration clearance mechanism
  // 并发时所有的限流请求竞争同一把锁，换并发锁
  rate_limit_cache: Mutex<HashMap<(i64, UserKey), DateTime<Utc>>>,
}

impl AppState {
  async fn is_admin(&self, user_id: i64) -> bool {
    self.admin_ids.lock().await.contains(&user_id)
  }

  async fn check_rate_limit(
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
    let mut cache = self.rate_limit_cache.lock().await;
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

  async fn preload_configs(&self) -> Result<(), AppError> {
    let sites = Sites::find_all(&self.conn).await.with_op("get all sites")?;
    for site in sites {
      self.site_config.lock().await.insert(site.id, site.config);
    }

    let users = Users::find()
      .all(&self.conn)
      .await
      .with_op("get all users")?;
    for user in users {
      if user.role == UserRole::Admin {
        self.admin_ids.lock().await.insert(user.id);
      }
    }
    Ok(())
  }

  async fn get_site_config(&self, site_id: i64) -> Result<SiteConfig, AppError> {
    {
      let cache = self.site_config.lock().await;
      if let Some(config) = cache.get(&site_id) {
        return Ok(config.clone());
      }
    }

    tracing::info!("Cache miss for site {}, fetching from DB", site_id);
    let all_sites = Sites::find_all(&self.conn).await.with_op("get all sites")?;
    let mut cache = self.site_config.lock().await;

    for site in all_sites {
      cache.insert(site.id, site.config);
    }

    cache
      .get(&site_id)
      .cloned()
      .ok_or_else(|| AppError::Internal {
        msg: format!("Site {} not found", site_id),
        source: None,
      })
  }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
  init_tracing();
  let config = Config::from_env()?;
  let conn = migrate(&config.database_url).await?;
  let addr = format!("{}:{}", config.host, config.port);
  let state = Arc::new(AppState {
    conn,
    jwt_key: config.jwt_key,
    site_config: Mutex::new(HashMap::new()),
    admin_ids: Mutex::new(HashSet::new()),
    rate_limit_cache: Mutex::new(HashMap::new()),
  });
  state.preload_configs().await?;
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
        .unwrap_or_else(|_| EnvFilter::new("info,sqlx::query=off,hyper=debug,axum=debug")),
    )
    .with(tracing_subscriber::fmt::layer().with_target(true))
    .init();
}

fn create_router(state: Arc<AppState>) -> Router {
  let public_routes = Router::new()
    .route("/health", get(health::health_check))
    .route("/auth/register", post(auth::register))
    .route("/auth/login", post(auth::login))
    .route("/comments", post(comment::create).get(comment::list))
    .route_layer(from_extractor_with_state::<OptionnalAuth, Arc<AppState>>(
      state.clone(),
    ));

  let private_routes = Router::new()
    .route("/users/me", get(user::profile).patch(user::update_profile))
    .route(
      "/sites",
      post(site::create).get(site::list).patch(site::update),
    )
    .route_layer(from_extractor_with_state::<RequireAuth, Arc<AppState>>(
      Arc::clone(&state),
    ));

  let api_routes = Router::new()
    .merge(public_routes)
    .merge(private_routes)
    .route_layer(from_extractor::<RemoteIp>());

  Router::new()
    .route("/static/yoin.js", get(handle_js))
    .nest("/api", api_routes)
    .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
    .layer(middleware::map_response(common_error_interceptor))
    .layer(CorsLayer::permissive())
    .with_state(state)
}

async fn handle_js(headers: HeaderMap) -> impl IntoResponse {
  let js = include_str!("../ui/dist/index.js");

  #[cfg(debug_assertions)]
  let (etag, cache_ctrl): (Option<&str>, &str) = (None, "no-store, must-revalidate");

  #[cfg(not(debug_assertions))]
  let (etag, cache_ctrl) = (
    Some(concat!("\"", env!("CARGO_PKG_VERSION"), "\"")),
    "no-cache",
  );

  if let Some(current_etag) = etag
    && let Some(if_none_match) = headers.get(header::IF_NONE_MATCH)
    && if_none_match == current_etag
  {
    return StatusCode::NOT_MODIFIED.into_response();
  }

  let mut builder = Response::builder()
    .header(header::CONTENT_TYPE, "application/javascript")
    .header(header::CACHE_CONTROL, cache_ctrl);

  if let Some(current_etag) = etag {
    builder = builder.header(header::ETAG, current_etag);
  }

  builder.body(Body::from(js)).unwrap().into_response()
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
