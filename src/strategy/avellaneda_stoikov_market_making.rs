use disruptor::{MultiProducer, SingleConsumerBarrier};

use crate::strategy::{
    common_data_representation::price_update::PriceUpdate,
    exchange::traits::Exchange,
};




pub struct AvellanedaStoikovMarketMaking<E> {
    pub exchange: E,
    pub symbols: Vec<String>,
    pub disruptor: MultiProducer<PriceUpdate, SingleConsumerBarrier>,
    // pub(crate) hyperliquid: super::exchange::hyperliquid::Hyperliquid,
}

impl<E: Exchange<PriceUpdate>> AvellanedaStoikovMarketMaking<E> {
    pub fn new(
        exchange: E,
        symbols: Vec<String>,
        disruptor: MultiProducer<PriceUpdate, SingleConsumerBarrier>,
    ) -> Self {
        Self {
            exchange,
            symbols,
            disruptor,
        }
    }

    pub async fn run(&self) {
        self.exchange.listen_trades(self.disruptor.clone()).await;
    }

}