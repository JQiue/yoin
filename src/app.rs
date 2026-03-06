use std::{
  collections::{HashMap, HashSet},
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
use migration::enums::UserRole;
use sea_orm::DatabaseConnection;
use tokio::sync::Mutex;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use crate::{
  entity::sites::SiteConfig,
  error::{AppError, ToAppError},
  extractor::{OptionnalAuth, RemoteIp, RequireAuth},
  handler::{auth, comment, health, site, user},
  helper::RateLimiter,
  repository::Repository,
  service::AppService,
};

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub enum UserKey {
  UserId(i64),
  Ip(String),
}

pub struct AppState {
  pub jwt_key: String,
  site_config: Mutex<HashMap<i64, SiteConfig>>,
  admin_ids: Mutex<HashSet<i64>>,
  // TODO: There is no expiration clearance mechanism
  // TODO: 并发时所有的限流请求竞争同一把锁，换并发锁
  pub comment_rate_limiter: RateLimiter<(i64, UserKey)>,
  pub service: AppService,
}

impl AppState {
  pub async fn is_admin(&self, user_id: i64) -> bool {
    self.admin_ids.lock().await.contains(&user_id)
  }

  pub async fn preload_configs(&self) -> Result<(), AppError> {
    let sites = self
      .service
      .repo
      .site()
      .find_all()
      .await
      .with_op("get all sites")?;
    let mut site_config = self.site_config.lock().await;
    site_config.clear();
    for site in sites {
      site_config.insert(site.id, site.config);
    }
    drop(site_config);
    let users = self
      .service
      .repo
      .user()
      .find_all()
      .await
      .with_op("get all users")?;
    let mut admin_ids = self.admin_ids.lock().await;
    admin_ids.clear();
    for user in users {
      if user.role == UserRole::Admin {
        admin_ids.insert(user.id);
      }
    }
    Ok(())
  }

  pub async fn get_site_config(&self, site_id: i64) -> Result<SiteConfig, AppError> {
    {
      let cache = self.site_config.lock().await;
      if let Some(config) = cache.get(&site_id) {
        return Ok(config.clone());
      }
    }

    tracing::info!("Cache miss for site {}, fetching from DB", site_id);
    let all_sites = self
      .service
      .repo
      .site()
      .find_all()
      .await
      .with_op("get all sites")?;
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

fn create_router(state: Arc<AppState>) -> Router {
  let public_routes = Router::new()
    .route("/health", get(health::health_check))
    .route("/auth/register", post(auth::register))
    .route("/auth/login", post(auth::login))
    .route("/auth/oauth/{provider}/start", get(auth::oauth_start))
    .route("/auth/oauth/{provider}/callback", get(auth::oauth_callback))
    .route("/comments", post(comment::create).get(comment::list))
    .route("/comments/{id}/replies", get(comment::list_replies))
    .route_layer(from_extractor_with_state::<OptionnalAuth, Arc<AppState>>(
      state.clone(),
    ));

  let private_routes = Router::new()
    .route("/users/me", get(user::profile).patch(user::update_profile))
    .route(
      "/sites",
      post(site::create).get(site::list).patch(site::update),
    )
    .route("/admin/oauth/providers", post(auth::create_oauth_provider))
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

async fn common_error_interceptor(res: Response) -> Response {
  let status = res.status();
  if status == StatusCode::METHOD_NOT_ALLOWED {
    let msg = "Method not allowed";
    return AppError::bad_request(msg.to_string()).into_response();
  }
  res
}

pub async fn app(conn: &'static DatabaseConnection, jwt_key: String) -> Result<Router, AppError> {
  let state = Arc::new(AppState {
    jwt_key,
    site_config: Mutex::new(HashMap::new()),
    admin_ids: Mutex::new(HashSet::new()),
    comment_rate_limiter: RateLimiter::new(),
    service: AppService {
      repo: Repository::new(conn),
    },
  });
  state.preload_configs().await?;
  Ok(create_router(state))
}
