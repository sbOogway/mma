use std::{
    sync::{Arc, OnceLock},
    time::Duration,
};

use async_trait::async_trait;
use disruptor::{
    MultiProducer, MultiProducerBarrier, ProcessorSettings, SingleConsumerBarrier, Sleep,
    builder::{NC, multi::MPBuilder},
};
use futures_util::future;
use tokio::sync::mpsc::{self, Sender};

use crate::{
    common_data_representation::message::Message,
    common_data_representation::mqtt::MqttPublisher,
    config::AppConfig,
    exchange::{self, Exchange},
    strategy::Strategy,
};

pub struct AvellanedaStoikovMarketMaking {}

impl AvellanedaStoikovMarketMaking {
    fn handle_message(message: &Message) {
        tracing::info!("{:#?}", message);
        let _ = MQTT_TX.get().unwrap().try_send(message.clone());
    }
}

pub static DISRUPTOR_PRODUCER: OnceLock<MultiProducer<Message, SingleConsumerBarrier>> =
    OnceLock::new();

pub static MQTT_TX: OnceLock<Sender<Message>> = OnceLock::new();
pub static EXCHANGES: OnceLock<Vec<Box<dyn Exchange>>> = OnceLock::new();

#[async_trait]
impl Strategy for AvellanedaStoikovMarketMaking {
    fn new(cfg: &AppConfig) -> Self {
        let (mqtt_tx, mqtt_rx) = mpsc::channel(256);
        let _mqtt_handle = tokio::spawn(MqttPublisher::run(cfg.mqtt.clone(), mqtt_rx));

        let disruptor_producer = disruptor::build_multi_producer(
            cfg.disruptor.buffer_size,
            || Message::empty(),
            Sleep::new(Duration::from_millis(1)),
        )
        .pin_at_core(1)
        .handle_events_with(|message, seq, batch| {
            AvellanedaStoikovMarketMaking::handle_message(message)
        })
        .build();

        let _ = MQTT_TX.set(mqtt_tx);
        let _ = DISRUPTOR_PRODUCER.set(disruptor_producer);
        let _ = EXCHANGES.set(
            cfg.runtime
                .exchanges
                .iter()
                .map(|name| exchange::new(name, cfg))
                .collect(),
        );

        Self {}
    }

    async fn run(&self) {
        for exchange in EXCHANGES.get().unwrap() {
            let producer = DISRUPTOR_PRODUCER.get().unwrap().clone();
            tokio::spawn(async move {
                exchange.listen(producer).await;
            });
        }
        future::pending::<()>().await;
    }
}
