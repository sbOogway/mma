use tracing_subscriber::EnvFilter;

use crate::{config::AppConfig, strategy::registry};

mod config;
mod strategy;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    strategy::register_strategies();

    let cfg = AppConfig::load()?;

    tracing::info!(?cfg, "configuration loaded");

    registry::run(&cfg.runtime.strategy, &cfg).await;

    Ok(())
}
