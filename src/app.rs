use std::{
  collections::{HashMap, HashSet},
  sync::Arc,
};

use axum::{
  Router,
  http::StatusCode,
  middleware::{self, from_extractor, from_extractor_with_state},
  response::{IntoResponse, Response},
  routing::{delete, get, patch, post},
};
use migration::enums::UserRoleBindingScopeType;
use sea_orm::DatabaseConnection;
use tokio::sync::Mutex;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use crate::{
  entity::sites::SiteConfig,
  error::{AppError, ToAppError},
  extractor::{OptionnalAuth, RemoteIp, RequireAuth, ensure_guest_id},
  handler::{admin, auth, comment, health, js, moderation, reaction, site, subscription, user},
  helper::RateLimiter,
  rbac::{bootstrap::bootstrap_rbac, permissions::roles::SUPER_ADMIN},
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
    let role = self
      .service
      .repo
      .role()
      .find_by_name(SUPER_ADMIN)
      .await
      .with_op("find super admin role")?;
    let mut admin_ids = self.admin_ids.lock().await;
    admin_ids.clear();
    if let Some(role) = role {
      let bindings = self
        .service
        .repo
        .user_role_binding()
        .find_all_by_role_and_scope(role.id, UserRoleBindingScopeType::Global, None)
        .await
        .with_op("find global super admin bindings")?;
      for binding in bindings {
        admin_ids.insert(binding.user_id);
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

fn create_router(state: Arc<AppState>) -> Router {
  let public_routes = Router::new()
    .route("/health", get(health::health_check))
    .route("/auth/register", post(auth::register))
    .route("/auth/login", post(auth::login))
    .route("/auth/external/exchange", post(auth::external_exchange))
    .route(
      "/auth/oauth/providers",
      get(auth::list_public_oauth_providers),
    )
    .route("/auth/oauth/{provider}/start", get(auth::oauth_start))
    .route("/auth/oauth/{provider}/callback", get(auth::oauth_callback))
    .route("/sites/{id}/config", get(site::public_config))
    .route("/comments", post(comment::create).get(comment::list))
    .route("/comments/{id}/replies", get(comment::list_replies))
    .route("/reactions", post(reaction::upsert).get(reaction::list))
    .route_layer(from_extractor_with_state::<OptionnalAuth, Arc<AppState>>(
      state.clone(),
    ));

  let private_routes = Router::new()
    .route("/users/me", get(user::profile).patch(user::update_profile))
    .route("/comments/{id}", delete(comment::delete))
    .route("/comments/{id}/sticky", patch(comment::set_sticky))
    .route(
      "/comment-subscriptions",
      get(subscription::list).post(subscription::create),
    )
    .route("/comment-subscriptions/{id}", delete(subscription::delete))
    .route(
      "/sites",
      post(site::create).get(site::list).patch(site::update),
    )
    .route(
      "/admin/oauth/providers",
      get(auth::list_oauth_providers).post(auth::create_oauth_provider),
    )
    .route(
      "/admin/oauth/providers/{id}",
      patch(auth::update_oauth_provider),
    )
    .route(
      "/admin/moderation/providers",
      get(moderation::list_providers).post(moderation::create_provider),
    )
    .route(
      "/admin/moderation/providers/{id}",
      patch(moderation::update_provider),
    )
    .route("/admin/me/capabilities", get(admin::me_capabilities))
    .route("/admin/users", get(admin::list_users))
    .route("/admin/rbac/roles", get(admin::list_roles))
    .route(
      "/admin/rbac/roles/{id}/permissions",
      patch(admin::replace_role_permissions),
    )
    .route("/admin/rbac/permissions", get(admin::list_permissions))
    .route(
      "/admin/rbac/user-role-bindings",
      get(admin::list_user_role_bindings).post(admin::create_user_role_binding),
    )
    .route(
      "/admin/rbac/user-role-bindings/{id}",
      delete(admin::delete_user_role_binding),
    )
    .route("/admin/comments", get(admin::list_comments))
    .route("/admin/comments/{id}", patch(admin::update_comment_status))
    .route_layer(from_extractor_with_state::<RequireAuth, Arc<AppState>>(
      Arc::clone(&state),
    ));

  let api_routes = Router::new()
    .merge(public_routes)
    .merge(private_routes)
    .route_layer(from_extractor::<RemoteIp>())
    .layer(middleware::from_fn(ensure_guest_id));

  Router::new()
    .route("/static/client.js", get(js::handle_client_js))
    .route("/static/admin.js", get(js::handle_admin_js))
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
    comment_rate_limiter: RateLimiter::default(),
    service: AppService {
      repo: Repository::new(conn),
    },
  });
  bootstrap_rbac(&state.service.repo).await?;
  state.preload_configs().await?;
  Ok(create_router(state))
}
