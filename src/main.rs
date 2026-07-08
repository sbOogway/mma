use tracing_subscriber::EnvFilter;

use crate::strategy::{
    avellaneda_stoikov_market_making::AvellanedaStoikovMarketMaking,
    common_data_representation::{disruptor::Disruptor, price_update::PriceUpdate},
    exchange::{hyperliquid::Hyperliquid, traits::DataProvider},
};

mod strategy;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let disruptor = Disruptor::new(
        64,
        || PriceUpdate::empty(),
        |update, seq, batch| update.handle_update(seq, batch),
    );

    let producer = disruptor.producer.clone();

    let hyperliquid = Hyperliquid::new(vec!["BTC".into(), "ETH".into(), "SOL".into()]);

    let asmm = AvellanedaStoikovMarketMaking::new(hyperliquid, vec!["BTC".into()], producer);

    asmm.run().await;
    

    Ok(())
}
