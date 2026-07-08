pub mod avellaneda_stoikov_market_making;
pub mod common_data_representation;
pub mod exchange;
pub mod registry;

pub fn register_strategies() {
    registry::register(
        "avellaneda_stoikov_market_making",
        |cfg| Box::pin(avellaneda_stoikov_market_making::AvellanedaStoikovMarketMaking::new(cfg).run()),
    );
}