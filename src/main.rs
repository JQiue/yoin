use std::net::SocketAddr;

use sea_orm::DatabaseConnection;
use tokio::net::TcpListener;
use tracing::{info, warn};
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};
use yoin::{app::app, config::Config, db};

#[cfg(debug_assertions)]
mod ui_dev;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
  init_tracing();
  let config = Config::from_env()?;

  match std::env::args().nth(1).as_deref() {
    None | Some("serve") => {}
    Some("migrate") => {
      db::migrate(&config.database_url).await?;
      info!("migrations applied");
      return Ok(());
    }
    Some(other) => {
      return Err(format!("unknown command `{other}`: expected `serve` or `migrate`").into());
    }
  }

  let connection = if config.migrate_on_start() {
    db::migrate(&config.database_url).await?
  } else {
    info!("YOIN_MIGRATE is off: skipping startup migrations, run `yoin migrate` to apply them");
    db::connect(&config.database_url).await?
  };
  let conn: &'static DatabaseConnection = Box::leak(Box::new(connection));
  let addr = format!("{}:{}", config.host, config.port);
  let app = app(conn, config.jwt_key)
    .await?
    .into_make_service_with_connect_info::<SocketAddr>();
  let listener = TcpListener::bind(&addr).await?;
  info!("Server running on http://{}", addr);

  #[cfg(debug_assertions)]
  let ui_dev_children = ui_dev::spawn();

  axum::serve(listener, app)
    .with_graceful_shutdown(shutdown_signal())
    .await?;

  #[cfg(debug_assertions)]
  ui_dev::stop(ui_dev_children);

  Ok(())
}

async fn shutdown_signal() {
  if let Err(error) = tokio::signal::ctrl_c().await {
    warn!("could not listen for Ctrl-C: {error}");
  }
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
