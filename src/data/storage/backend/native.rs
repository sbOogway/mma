use std::collections::HashMap;
use std::sync::RwLock;

use std::collections::VecDeque;
use std::time::{Duration, Instant};

use crate::data::storage::expiration_buffer::ExpirationBuffer;
use crate::data::storage::memory_map::MemoryMap;

pub struct NativeMemoryMap<V> {
    inner: RwLock<HashMap<String, V>>,
}

impl<V> Default for NativeMemoryMap<V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<V> NativeMemoryMap<V> {
    pub fn new() -> Self {
        Self {
            inner: RwLock::new(HashMap::new()),
        }
    }
}

impl<V: Clone + Send + Sync> MemoryMap<V> for NativeMemoryMap<V> {
    fn set(&self, key: String, value: V) {
        self.inner.write().unwrap().insert(key, value);
    }

    fn get(&self, key: &str) -> Option<V> {
        self.inner.read().unwrap().get(key).cloned()
    }
}

pub struct NativeExpirationBuffer<V> {
    ttl: Duration,
    inner: RwLock<VecDeque<(Instant, V)>>,
}

impl<V> NativeExpirationBuffer<V> {
    pub fn new(ttl: Duration) -> Self {
        Self {
            ttl,
            inner: RwLock::new(VecDeque::new()),
        }
    }
}

impl<V: Clone + Send + Sync + 'static> ExpirationBuffer<V> for NativeExpirationBuffer<V> {
    fn add(&self, value: V) {
        let mut list = self.inner.write().unwrap();
        list.push_back((Instant::now(), value));
    }

    fn get(&self) -> Box<dyn Iterator<Item = V>> {
        let mut list = self.inner.write().unwrap();

        while let Some(front) = list.front() {
            match self.ttl {
                ttl if front.0.elapsed() >= ttl => {
                    list.pop_front();
                }
                _ => break,
            }
        }

        let values: Vec<V> = list.iter().map(|(_, v)| v.clone()).collect();
        Box::new(values.into_iter())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_and_get_returns_all_elements() {
        let buf = NativeExpirationBuffer::new(Duration::from_millis(500));
        buf.add(1);
        buf.add(2);
        buf.add(3);
        assert_eq!(buf.get().collect::<Vec<_>>(), vec![1, 2, 3]);
    }

    #[test]
    fn get_returns_empty_when_empty() {
        let buf = NativeExpirationBuffer::<i32>::new(Duration::from_secs(2));
        assert!(buf.get().collect::<Vec<_>>().is_empty());
    }

    #[test]
    fn elements_expire_after_ttl() {
        let buf = NativeExpirationBuffer::new(Duration::from_millis(10));
        buf.add(42);
        assert_eq!(buf.get().collect::<Vec<_>>(), vec![42]);
        std::thread::sleep(Duration::from_millis(20));
        assert!(buf.get().collect::<Vec<_>>().is_empty());
    }

    #[test]
    fn expired_elements_dont_block_fresh_ones() {
        let buf = NativeExpirationBuffer::new(Duration::from_millis(10));
        buf.add(1);
        buf.add(2);
        std::thread::sleep(Duration::from_millis(20));
        buf.add(3);
        assert_eq!(buf.get().collect::<Vec<_>>(), vec![3]);
    }
}
