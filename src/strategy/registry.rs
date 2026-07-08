use std::{
    collections::HashMap,
    future::Future,
    pin::Pin,
    sync::{LazyLock, Mutex},
};

use crate::config::AppConfig;

type BoxFuture<'a> = Pin<Box<dyn Future<Output = ()> + Send + 'a>>;
type Runner = for<'a> fn(&'a AppConfig) -> BoxFuture<'a>;

static REGISTRY: LazyLock<Mutex<HashMap<&'static str, Runner>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

pub fn register(name: &'static str, runner: Runner) {
    REGISTRY.lock().unwrap().insert(name, runner);
}

pub async fn run(name: &str, cfg: &AppConfig) {
    let runner = *REGISTRY
        .lock()
        .unwrap()
        .get(name)
        .unwrap_or_else(|| panic!("unknown strategy: {name}"));
    runner(cfg).await;
}
