//! Development-only companion: starts the frontend dev servers next to the API.
//!
//! Enabled for debug builds only, so release binaries and containers never spawn node.
//! `YOIN_UI_DEV` picks the playgrounds: `client` and `admin` by default, or a subset such as
//! `YOIN_UI_DEV=admin`, or `YOIN_UI_DEV=0` to start the API alone.

use std::{
  net::{Ipv4Addr, SocketAddr, TcpStream},
  path::Path,
  process::{Child, Command, Stdio},
  time::Duration,
};

use tracing::{info, warn};

/// Playground entry point and port, in start order.
const PLAYGROUNDS: [(&str, u16); 2] = [("client", 3000), ("admin", 3001)];
const ENV_VAR: &str = "YOIN_UI_DEV";

/// Spawns one rsbuild dev server per requested playground, skipping ports that are already taken.
pub fn spawn() -> Vec<Child> {
  let Some(playgrounds) = requested_playgrounds() else {
    info!("UI dev server disabled by {ENV_VAR}");
    return Vec::new();
  };

  let ui_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("ui");
  // Same binary the npm scripts invoke, minus `--open`: an already open tab keeps its hot
  // reload, and `cargo run` should not launch browsers on its own.
  let rsbuild = ui_dir.join("node_modules/.bin").join(if cfg!(windows) {
    "rsbuild.cmd"
  } else {
    "rsbuild"
  });

  let mut children = Vec::new();
  for (environment, port) in playgrounds {
    let address = SocketAddr::from((Ipv4Addr::LOCALHOST, port));
    if TcpStream::connect_timeout(&address, Duration::from_millis(200)).is_ok() {
      info!("port {port} is already taken; assuming the {environment} playground runs there");
      continue;
    }

    let port_arg = port.to_string();
    match Command::new(&rsbuild)
      .args(["dev", "--environment", environment, "--port", &port_arg])
      .current_dir(&ui_dir)
      .spawn()
    {
      Ok(child) => {
        info!("{environment} playground running on http://localhost:{port}/{environment}");
        children.push(child);
      }
      Err(error) => warn!(
        "could not start the {environment} playground ({error}); run `npm install` in ui/ or set {ENV_VAR}=0"
      ),
    }
  }

  children
}

/// Kills every dev server and its children.
pub fn stop(children: Vec<Child>) {
  for child in children {
    #[cfg(windows)]
    let killed = Command::new("taskkill")
      .args(["/PID", &child.id().to_string(), "/T", "/F"])
      .stdout(Stdio::null())
      .stderr(Stdio::null())
      .status();

    #[cfg(not(windows))]
    let killed = Command::new("kill")
      .args(["-TERM", &child.id().to_string()])
      .status();

    if killed.is_err() {
      warn!("could not stop a UI dev server");
    }
  }
}

/// Playgrounds selected by `YOIN_UI_DEV`, or `None` when they are switched off.
fn requested_playgrounds() -> Option<Vec<(&'static str, u16)>> {
  let Ok(value) = std::env::var(ENV_VAR) else {
    return Some(PLAYGROUNDS.to_vec());
  };

  let requested: Vec<&str> = value.split(',').map(str::trim).collect();
  let selected: Vec<(&str, u16)> = PLAYGROUNDS
    .into_iter()
    .filter(|(environment, _)| requested.contains(environment))
    .collect();

  if selected.is_empty() {
    None
  } else {
    Some(selected)
  }
}
