//! `expiration_buffer` module provides the `ExpirationBuffer` trait and factory for
//! time-windowed value accumulation with automatic TTL-based eviction.

use std::time::Duration;

use crate::config::AppConfig;
use crate::data::storage::backend::native::NativeExpirationBuffer;
use crate::data::storage::backend::redis::RedisExpirationBuffer;

pub trait ExpirationBuffer<V>: Send + Sync {
    fn add(&self, value: V);
    fn get(&self) -> Option<Box<dyn Iterator<Item = V>>>;
}

pub fn new<V: Clone + serde::Serialize + serde::de::DeserializeOwned + Send + Sync + 'static>(
    backend: &str,
    ttl: Duration,
    cfg: Option<&AppConfig>,
) -> Box<dyn ExpirationBuffer<V>> {
    match backend {
        "native" => Box::new(NativeExpirationBuffer::new(ttl)),
        "redis" => Box::new(RedisExpirationBuffer::new(
            &cfg.unwrap()
                .memory_storage
                .redis
                .as_ref()
                .expect("redis config required for redis backend")
                .socket_path,
            ttl,
        )),
        other => panic!("unknown ttl buffer backend: {other}"),
    }
}
