use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub runtime: RuntimeConfig,
    pub exchange: ExchangeConfigs,
    pub strategy: StrategyConfigs,
    pub disruptor: DisruptorConfig,
}

#[derive(Debug, Deserialize)]
pub struct RuntimeConfig {
    pub exchanges: Vec<String>,
    pub strategy: String,
}

#[derive(Debug, Deserialize)]
pub struct ExchangeConfigs {
    pub hyperliquid: Option<HyperliquidConfig>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct HyperliquidConfig {
    pub coins: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct DisruptorConfig {
    pub buffer_size: usize,
}

#[derive(Debug, Deserialize)]
pub struct StrategyConfigs {
    pub avellaneda_stoikov_market_making: Option<AvellanedaStoikovConfig>,
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
