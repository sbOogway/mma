//! `expiration_buffer` module provides the `ExpirationBuffer` trait and factory for
//! time-windowed value accumulation with automatic TTL-based eviction.

use std::time::Duration;

use crate::data::storage::backend::native::NativeExpirationBuffer;

pub trait ExpirationBuffer<V>: Send + Sync {
    fn add(&self, value: V);
    fn get(&self) -> Option<Box<dyn Iterator<Item = V>>>;
}

pub fn new<V: Clone + Send + Sync + 'static>(
    backend: &str,
    ttl: Duration,
) -> Box<dyn ExpirationBuffer<V>> {
    match backend {
        "native" => Box::new(NativeExpirationBuffer::new(ttl)),
        other => panic!("unknown ttl buffer backend: {other}"),
    }
}
