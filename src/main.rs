use tracing_subscriber::EnvFilter;

use crate::{
    config::AppConfig,
    strategy::{
        avellaneda_stoikov_market_making::AvellanedaStoikovMarketMaking,
        common_data_representation::{disruptor::Disruptor, price_update::PriceUpdate},
        exchange::hyperliquid::Hyperliquid,
    },
};

mod config;
mod strategy;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let cfg = AppConfig::load()?;

    tracing::info!(?cfg, "configuration loaded");

    let disruptor = Disruptor::new(
        cfg.disruptor.buffer_size,
        || PriceUpdate::empty(),
        |update, seq, batch| update.handle(seq, batch),
    );

    let producer = disruptor.producer.clone();

    let hyperliquid = Hyperliquid::new(cfg.exchange.hyperliquid.coins);

    let asmm = AvellanedaStoikovMarketMaking::new(hyperliquid, producer);

    asmm.run().await;

    Ok(())
}
