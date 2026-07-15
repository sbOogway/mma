use std::collections::VecDeque;
use std::sync::RwLock;
use std::time::{Duration, Instant};

use super::ExpirationBuffer;

pub struct ExpirationVec<V> {
    ttl: Option<Duration>,
    inner: RwLock<VecDeque<(Instant, V)>>,
}

impl<V> ExpirationVec<V> {
    pub fn new(ttl: Option<Duration>) -> Self {
        Self {
            ttl,
            inner: RwLock::new(VecDeque::new()),
        }
    }
}

impl<V: Clone + Send + Sync + 'static> ExpirationBuffer<V> for ExpirationVec<V> {
    fn add(&self, value: V) {
        let mut list = self.inner.write().unwrap();
        list.push_back((Instant::now(), value));
    }

    fn get(&self) -> Option<Box<dyn Iterator<Item = V>>> {
        let mut list = self.inner.write().unwrap();

        while let Some(front) = list.front() {
            match self.ttl {
                Some(ttl) if front.0.elapsed() >= ttl => {
                    list.pop_front();
                }
                _ => break,
            }
        }

        if list.is_empty() {
            return None;
        }

        let values: Vec<V> = list.iter().map(|(_, v)| v.clone()).collect();
        Some(Box::new(values.into_iter()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_and_get_returns_all_elements() {
        let buf = ExpirationVec::new(None);
        buf.add(1);
        buf.add(2);
        buf.add(3);
        assert_eq!(buf.get().map(|i| i.collect::<Vec<_>>()), Some(vec![1, 2, 3]));
    }

    #[test]
    fn get_returns_none_when_empty() {
        let buf = ExpirationVec::<i32>::new(None);
        assert_eq!(buf.get().map(|i| i.collect::<Vec<_>>()), None);
    }

    #[test]
    fn elements_expire_after_ttl() {
        let buf = ExpirationVec::new(Some(Duration::from_millis(10)));
        buf.add(42);
        assert_eq!(buf.get().map(|i| i.collect::<Vec<_>>()), Some(vec![42]));
        std::thread::sleep(Duration::from_millis(20));
        assert_eq!(buf.get().map(|i| i.collect::<Vec<_>>()), None);
    }

    #[test]
    fn expired_elements_dont_block_fresh_ones() {
        let buf = ExpirationVec::new(Some(Duration::from_millis(10)));
        buf.add(1);
        buf.add(2);
        std::thread::sleep(Duration::from_millis(20));
        buf.add(3);
        assert_eq!(buf.get().map(|i| i.collect::<Vec<_>>()), Some(vec![3]));
    }
}
