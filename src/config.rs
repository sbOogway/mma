use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub exchange: ExchangeConfig,
    pub disruptor: DisruptorConfig,
    pub strategy: StrategyConfig,
}

#[derive(Debug, Deserialize)]
pub struct ExchangeConfig {
    pub hyperliquid: HyperliquidConfig,
}

#[derive(Debug, Deserialize)]
pub struct HyperliquidConfig {
    pub coins: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct DisruptorConfig {
    pub buffer_size: usize,
}

#[derive(Debug, Deserialize)]
pub struct StrategyConfig {
    pub avellaneda_stoikov_market_making: AvellanedaStoikovConfig,
}

#[derive(Debug, Deserialize)]
pub struct AvellanedaStoikovConfig {
    pub gamma: f64,
    pub q: f64,
    pub sigma: f64,
}

impl AppConfig {
    pub fn load() -> Result<Self, config::ConfigError> {
        config::Config::builder()
            .add_source(config::File::with_name("config/default"))
            .add_source(config::File::with_name("config/local").required(false))
            .add_source(config::Environment::with_prefix("MMA"))
            .build()?
            .try_deserialize()
    }
}
