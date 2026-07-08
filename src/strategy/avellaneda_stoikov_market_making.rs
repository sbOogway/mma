use disruptor::{MultiProducer, SingleConsumerBarrier};

use crate::strategy::{
    common_data_representation::price_update::PriceUpdate,
    exchange::traits::Exchange,
};

pub struct AvellanedaStoikovMarketMaking<E> {
    pub exchange: E,
    pub disruptor: MultiProducer<PriceUpdate, SingleConsumerBarrier>,
}

impl<E: Exchange<PriceUpdate>> AvellanedaStoikovMarketMaking<E> {
    pub fn new(
        exchange: E,
        disruptor: MultiProducer<PriceUpdate, SingleConsumerBarrier>,
    ) -> Self {
        Self {
            exchange,
            disruptor,
        }
    }

    pub async fn run(&self) {
        self.exchange.listen_trades(self.disruptor.clone()).await;
    }

}