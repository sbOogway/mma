use std::collections::HashMap;
use std::fmt::Display;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

use tokio::sync::mpsc;

use crate::data::storage::memory_map::MemoryMap;

pub struct RedisMemoryMap<V> {
    cache: Arc<RwLock<HashMap<String, (V, Option<Instant>)>>>,
    _tx: mpsc::UnboundedSender<(String, V, Option<Duration>)>,
}

impl<V: Display + Send + Sync + 'static> RedisMemoryMap<V> {
    pub fn new(socket_path: &str) -> Self {
        let cache = Arc::new(RwLock::new(HashMap::new()));
        let (tx, rx) = mpsc::unbounded_channel();
        let cache_clone = cache.clone();
        let path = socket_path.to_string();
        tokio::spawn(async move {
            Self::background_worker(path, rx, cache_clone).await;
        });
        Self { cache, _tx: tx }
    }

    async fn background_worker(
        socket_path: String,
        mut rx: mpsc::UnboundedReceiver<(String, V, Option<Duration>)>,
        _cache: Arc<RwLock<HashMap<String, (V, Option<Instant>)>>>,
    ) {
        let client = match redis::Client::open(format!("redis+unix://{}", socket_path)) {
            Ok(c) => c,
            Err(e) => {
                tracing::error!(error = %e, "invalid redis url");
                return;
            }
        };

        let mut conn = loop {
            match client.get_connection_manager().await {
                Ok(cm) => break cm,
                Err(e) => {
                    tracing::warn!(error = %e, "redis connection failed, retrying in 1s");
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
            }
        };

        while let Some((key, value, ttl)) = rx.recv().await {
            let value_str = value.to_string();
            let result = if let Some(ttl) = ttl {
                let secs = ttl.as_secs();
                redis::cmd("SET")
                    .arg(&[&key, &value_str, "EX", &secs.to_string()])
                    .query_async::<()>(&mut conn)
                    .await
            } else {
                redis::cmd("SET")
                    .arg(&[&key, &value_str])
                    .query_async::<()>(&mut conn)
                    .await
            };
            if let Err(e) = result {
                tracing::warn!(error = %e, key = %key, "redis set failed");
                match client.get_connection_manager().await {
                    Ok(new_conn) => conn = new_conn,
                    Err(e) => tracing::warn!(error = %e, "redis reconnection failed"),
                }
            }
        }
    }
}

impl<V: Display + Clone + Send + Sync + 'static> MemoryMap<V> for RedisMemoryMap<V> {
    fn set(&self, key: String, value: V) {}

    fn get(&self, key: &str) -> Option<V> {
        let map = self.cache.read().unwrap();
        match map.get(key) {
            Some((_, Some(expires))) if *expires <= Instant::now() => None,
            Some((value, _)) => Some(value.clone()),
            None => None,
        }
    }
}
