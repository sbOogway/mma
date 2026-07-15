//! `memory_storage` module is responsible for managing connection to various
//! memory backends, such as redis and various rust hashmaps. provides a single 
//! interchangeable api to easily switch among them.
//! also contains other data structures such as buffer with automatic element
//! expiration.

pub mod hashmap_storage;
pub mod redis_storage;
pub mod expiration_vec;

use std::fmt::Display;
use std::time::Duration;

use self::{
    hashmap_storage::HashMapStorage, redis_storage::RedisStorage, expiration_vec::ExpirationVec,
};

pub trait MemoryMapStorage<V>: Send + Sync {
    fn set(&self, key: String, value: V);
    fn get(&self, key: &str) -> Option<V>;
}

pub trait ExpirationBuffer<V>: Send + Sync {
    fn add(&self, value: V);
    fn get(&self) -> Option<Box<dyn Iterator<Item = V>>>;
}

pub fn new<V: Clone + Display + Send + Sync + 'static>(
    cfg: &crate::config::MemoryStorageConfig,
    ttl: Option<Duration>,
) -> Box<dyn MemoryMapStorage<V>> {
    match cfg.backend.as_str() {
        "hashmap" => Box::new(HashMapStorage::new()),
        "redis" => Box::new(RedisStorage::new(
            ttl,
            &cfg.redis
                .as_ref()
                .expect("redis config required for redis backend")
                .socket_path,
        )),
        other => panic!("unknown memory_storage backend: {other}"),
    }
}

pub fn new_ttl_buffer<V: Clone + Send + Sync + 'static>(
    cfg: &crate::config::MemoryStorageConfig,
    ttl: Option<Duration>,
) -> Box<dyn ExpirationBuffer<V>> {
    match cfg.backend.as_str() {
        "ttl_vec" => Box::new(ExpirationVec::new(ttl)),
        other => panic!("unknown ttl buffer backend: {other}"),
    }
}
