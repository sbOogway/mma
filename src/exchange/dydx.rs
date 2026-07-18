use crate::{
    ccxt::{CcxtOrderSide, CcxtTrade},
    config::DydxConfig,
    utils::{self, big_decimal_to_decimal},
};
use async_trait::async_trait;
use bigdecimal::BigDecimal;
use dydx::{
    indexer::{
        Feed, IndexerClient, IndexerConfig, OrderSide, RestConfig, SockConfig, Ticker,
        TradesMessage,
    },
    node::{Subaccount, Wallet},
};
use rust_decimal::Decimal;
use serde_json::Value::Null;

use crate::ccxt::{self, Ccxt};
use tokio::sync::Mutex;

impl From<OrderSide> for CcxtOrderSide {
    fn from(value: OrderSide) -> Self {
        match value {
            OrderSide::Buy => Self::Buy,
            OrderSide::Sell => Self::Sell,
        }
    }
}

pub struct Dydx {
    indexer: Mutex<IndexerClient>,
    trades_feed: Option<Feed<TradesMessage>>,
}

impl Dydx {
    pub fn new(cfg: &DydxConfig) -> Self {
        let sock_cfg = SockConfig {
            endpoint: cfg.indexer_ws_endpoint.clone(),
            timeout: 5_000,
            rate_limit: std::num::NonZeroU32::new(2).unwrap(),
        };
        let rest_cfg = RestConfig {
            endpoint: "http://localhost".into(),
        };
        let indexer_cfg = IndexerConfig {
            rest: rest_cfg,
            sock: sock_cfg,
        };

        let indexer = IndexerClient::new(indexer_cfg);

        let wallet = match Wallet::from_mnemonic(&cfg.mnemonic) {
            Ok(w) => w,
            Err(e) => {
                tracing::error!(error = %e, "failed to create wallet from mnemonic");
                panic!();
            }
        };
        let account = match wallet.account_offline(0) {
            Ok(a) => a,
            Err(e) => {
                tracing::error!(error = %e, "failed to derive account");
                panic!();
            }
        };
        tracing::info!(address = %account.address(), "dydx wallet derived");
        let subaccount: Subaccount = match account.subaccount(cfg.subaccount_number) {
            Ok(s) => s,
            Err(e) => {
                tracing::error!(error = %e, "failed to create subaccount");
                panic!();
            }
        };
        Self {
            indexer: Mutex::new(indexer),
            trades_feed: None,
        }
    }
}

#[async_trait]
impl Ccxt for Dydx {
    async fn load_markets(&mut self) {
        let mut tickers = Vec::<Ticker>::new();
        tickers.push(Ticker("BTC-USD".into()));

        let mut indexer = self.indexer.lock().await;
        let feed = indexer
            .feed()
            .trades(tickers.get(0).unwrap(), false)
            .await
            .expect("failed to get feed");

        self.trades_feed = Some(feed);
    }

    async fn watch_trades(
        &mut self,
        symbol: String,
        _since: Option<u64>,
        _limit: Option<u64>,
    ) -> Vec<ccxt::CcxtTrade> {
        let trades_feed = self.trades_feed.as_mut().unwrap();
        match trades_feed.recv().await {
            Some(dydx::indexer::TradesMessage::Initial(trades)) => trades
                .contents
                .trades
                .iter()
                .map(|trade| CcxtTrade {
                    info: Null,
                    id: trade.id.0.clone(),
                    timestamp: trade.created_at.timestamp_millis(),
                    datetime: trade.created_at,
                    symbol: symbol.clone(),
                    order: Some(trade.id.0.clone()),
                    order_type: None,
                    side: Some(trade.side.clone().into()),
                    taker_or_maker: None,
                    price: big_decimal_to_decimal(trade.price.0.clone()),
                    amount: big_decimal_to_decimal(trade.size.0.clone()),
                    cost: None,
                    fee: None,
                    fees: None,
                })
                .collect(),
            Some(dydx::indexer::TradesMessage::Update(trades)) => {
                let trades_update_contents = trades.contents;

                trades_update_contents
                    .iter()
                    .map(|update| {
                        update
                            .trades
                            .iter()
                            .map(|trade| CcxtTrade {
                                info: Null,
                                id: trade.id.0.clone(),
                                timestamp: trade.created_at.timestamp_millis(),
                                datetime: trade.created_at,
                                symbol: symbol.clone(),
                                order: Some(trade.id.0.clone()),
                                order_type: None,
                                side: Some(trade.side.clone().into()),
                                taker_or_maker: None,
                                price: big_decimal_to_decimal(trade.price.0.clone()),
                                amount: big_decimal_to_decimal(trade.price.0.clone()),
                                cost: None,
                                fee: None,
                                fees: None,
                            })
                            .collect::<Vec<CcxtTrade>>()
                    })
                    .flatten()
                    .collect()
            }

            None => Vec::new(),
        }
    }

    async fn watch_order_book(
        &self,
        symbols: Vec<String>,
        limit: Option<u8>,
    ) -> ccxt::CcxtOrderBook {
        todo!()
    }

    async fn watch_balance(&self) -> ccxt::CcxtBalance {
        todo!()
    }

    async fn watch_orders(
        &self,
        symbol: String,
        since: Option<u64>,
        limit: Option<u64>,
    ) -> ccxt::CcxtOrder {
        todo!()
    }

    async fn watch_my_trades(
        &self,
        symbols: Vec<String>,
        since: Option<u64>,
        limit: Option<u64>,
    ) -> ccxt::CcxtTrade {
        todo!()
    }

    async fn watch_positions(&self, symbols: Vec<String>) -> ccxt::CcxtPosition {
        todo!()
    }

    async fn create_order_ws(
        &self,
        symbol: String,
        type_: ccxt::CcxtOrderType,
        side: ccxt::CcxtOrderSide,
        amount: rust_decimal::prelude::Decimal,
        price: Option<rust_decimal::prelude::Decimal>,
    ) -> ccxt::CcxtOrder {
        todo!()
    }

    async fn edit_order_ws(
        &self,
        id: String,
        symbol: Option<String>,
        type_: Option<ccxt::CcxtOrderType>,
        side: Option<ccxt::CcxtOrderSide>,
        amount: Option<rust_decimal::prelude::Decimal>,
        price: Option<rust_decimal::prelude::Decimal>,
    ) -> ccxt::CcxtOrder {
        todo!()
    }

    async fn cancel_orders_ws(&self, id: String) -> ccxt::CcxtOrder {
        todo!()
    }

    async fn cancel_all_orders_ws(&self) {
        todo!()
    }
}
