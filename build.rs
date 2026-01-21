use std::process::Command;

use tracing::{Level, info};
use tracing_subscriber::FmtSubscriber;

fn main() {
  let subscriber = FmtSubscriber::builder()
    .with_max_level(Level::INFO)
    .finish();
  tracing::subscriber::set_global_default(subscriber).expect("Failed to set default subscriber");
  info!("build ui project");
  let cmd = if cfg!(windows) { "npm.cmd" } else { "npm" };
  let status = Command::new(cmd)
    .args(["run", "build"])
    .current_dir("ui")
    .status()
    .expect("npm run command failed");

  if !status.success() {
    panic!("npm run command failed");
  }
}
