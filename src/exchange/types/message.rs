pub mod asmm_quote;
pub mod fill_update;
pub mod position_update;
pub mod bbo_update;
pub mod trade_update;

pub use asmm_quote::AsmmQuote;
pub use position_update::{PositionUpdate};
pub use bbo_update::BboUpdate;
pub use trade_update::TradeUpdate;
pub use fill_update::FillUpdate;

#[derive(Clone, Debug)]
pub enum Message {
    Empty,
    TradeUpdate(TradeUpdate),
    BboUpdate(BboUpdate),
    AsmmQuote(AsmmQuote),
    BalanceUpdate(PositionUpdate),
    FillUpdate(FillUpdate)
}

impl Message {
    pub fn empty() -> Self {
        Self::Empty
    }
}