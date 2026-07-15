//! `memory_map` module provides the `MemoryMap` trait and factory for key-value storage backends.

use std::fmt::Display;

use crate::{
    config::AppConfig,
    data::storage::backend::{native::NativeMemoryMap, redis::RedisMemoryMap},
};

pub trait MemoryMap<V>: Send + Sync {
    fn set(&self, key: String, value: V);
    fn get(&self, key: &str) -> Option<V>;
}
pub fn new<V: Clone + Display + Send + Sync + 'static>(
    backend: &str,
    cfg: Option<&AppConfig>,
) -> Box<dyn MemoryMap<V>> {
    match backend {
        "native" => Box::new(NativeMemoryMap::new()),
        "redis" => Box::new(RedisMemoryMap::new(
            &cfg.unwrap()
                .memory_storage
                .redis
                .as_ref()
                .expect("redis config required for redis backend")
                .socket_path,
        )),
        other => panic!("unknown memory_storage backend: {other}"),
    }
}
