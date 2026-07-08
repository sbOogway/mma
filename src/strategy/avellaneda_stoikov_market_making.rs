use futures_util::future;

use crate::{
    config::AppConfig,
    strategy::{
        common_data_representation::{
            disruptor::Disruptor,
            price_update::PriceUpdate,
        },
        exchange::{traits::Exchange, create_exchange},
    },
};

pub struct AvellanedaStoikovMarketMaking {
    exchanges: Vec<Box<dyn Exchange<PriceUpdate>>>,
    producer: disruptor::MultiProducer<PriceUpdate, disruptor::SingleConsumerBarrier>,
}

impl AvellanedaStoikovMarketMaking {
    pub fn new(cfg: &AppConfig) -> Self {
        let d = Disruptor::new(
            cfg.disruptor.buffer_size,
            || PriceUpdate::empty(),
            |update, seq, batch| update.handle(seq, batch),
        );
        Self {
            exchanges: cfg
                .runtime
                .exchanges
                .iter()
                .map(|name| create_exchange(name, cfg))
                .collect(),
            producer: d.producer,
        }
    }

    pub async fn run(self) {
        for exchange in self.exchanges {
            let producer = self.producer.clone();
            tokio::spawn(async move {
                exchange.listen_trades(producer).await;
            });
        }
        future::pending::<()>().await;
    }
}
