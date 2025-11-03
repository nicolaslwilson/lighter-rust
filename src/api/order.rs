#![allow(clippy::too_many_arguments)]
use crate::{
    apis::{self, configuration::Configuration},
    config::LighterConfig,
    models::{
        order::{TimeInForce, Type as OrderType},
        ExchangeStats, ExportData, OrderBookDetails, OrderBookOrders, OrderBooks, Orders, Trades,
    },
    Result,
};

#[derive(Debug, Clone, strum::Display)]
#[strum(serialize_all = "snake_case")]
pub enum ExportType {
    Funding,
    Trade,
}

#[derive(Debug, Clone, strum::Display)]
#[strum(serialize_all = "snake_case")]
pub enum TradesSortBy {
    BlockHeight,
    Timestamp,
    TradeId,
}

#[derive(Debug, Clone, strum::Display)]
#[strum(serialize_all = "snake_case")]
pub enum TradesSortDir {
    Desc,
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone)]
#[repr(u8)]
pub enum GroupingType {
    OneTriggersOther = 1,
    OneCancelsOther = 2,
    OneTriggersAndOneCancelsOther = 3,
}

impl From<crate::models::order::Type> for u8 {
    fn from(val: crate::models::order::Type) -> Self {
        match val {
            OrderType::Limit => 0,
            OrderType::Market => 1,
            OrderType::StopLoss => 2,
            OrderType::StopLossLimit => 3,
            OrderType::TakeProfit => 4,
            OrderType::TakeProfitLimit => 5,
            OrderType::Twap => 6,
            OrderType::TwapSub => 7,
            OrderType::Liquidation => 8,
        }
    }
}

impl From<crate::models::order::TimeInForce> for u8 {
    fn from(val: crate::models::order::TimeInForce) -> Self {
        match val {
            TimeInForce::ImmediateOrCancel => 0,
            TimeInForce::GoodTillTime => 1,
            TimeInForce::PostOnly => 2,
            TimeInForce::Unknown => 3,
        }
    }
}

#[derive(Debug)]
pub struct OrderApi {
    config: apis::configuration::Configuration,
}

impl OrderApi {
    pub fn new(config: &LighterConfig) -> Result<Self> {
        Ok(Self {
            config: Configuration::try_from(config)?,
        })
    }

    /// Get account active orders. `auth` can be generated using the SDK.
    pub async fn account_active_orders(
        &self,
        account_index: i64,
        market_id: i32,
        authorization: Option<&str>,
        auth: Option<&str>,
    ) -> Result<Orders> {
        let resp = apis::order_api::account_active_orders(
            &self.config,
            account_index,
            market_id,
            authorization,
            auth,
        )
        .await
        .inspect_err(|e| tracing::error!("unable to call `account_active_orders`: {e}"))?;

        Ok(resp)
    }

    /// Get account inactive orders
    pub async fn account_inactive_orders(
        &self,
        account_index: i64,
        limit: i64,
        authorization: Option<&str>,
        auth: Option<&str>,
        market_id: Option<i32>,
        ask_filter: Option<i32>,
        between_timestamps: Option<&str>,
        cursor: Option<&str>,
    ) -> Result<Orders> {
        let resp = apis::order_api::account_inactive_orders(
            &self.config,
            account_index,
            limit,
            authorization,
            auth,
            market_id,
            ask_filter,
            between_timestamps,
            cursor,
        )
        .await
        .inspect_err(|e| tracing::error!("unable to call `account_inactive_orders`: {e}"))?;

        Ok(resp)
    }

    /// Get exchange stats
    pub async fn exchange_stats(&self) -> Result<ExchangeStats> {
        let resp = apis::order_api::exchange_stats(&self.config)
            .await
            .inspect_err(|e| tracing::error!("unable to call `exchange_stats`: {e}"))?;

        Ok(resp)
    }

    /// Export data
    pub async fn export(
        &self,
        export_type: ExportType,
        authorization: Option<&str>,
        auth: Option<&str>,
        account_index: Option<i64>,
        market_id: Option<i32>,
    ) -> Result<ExportData> {
        let resp = apis::order_api::export(
            &self.config,
            &export_type.to_string(),
            authorization,
            auth,
            account_index,
            market_id,
        )
        .await
        .inspect_err(|e| tracing::error!("unable to call `export`: {e}"))?;

        Ok(resp)
    }

    /// Get order books metadata
    pub async fn order_book_details(&self, market_id: Option<i32>) -> Result<OrderBookDetails> {
        let resp = apis::order_api::order_book_details(&self.config, market_id)
            .await
            .inspect_err(|e| tracing::error!("unable to call `order_book_details`: {e}"))?;

        Ok(resp)
    }

    /// Get order book orders
    pub async fn order_book_orders(&self, market_id: i32, limit: i64) -> Result<OrderBookOrders> {
        let resp = apis::order_api::order_book_orders(&self.config, market_id, limit)
            .await
            .inspect_err(|e| tracing::error!("unable to call `order_book_orders`: {e}"))?;

        Ok(resp)
    }

    /// Get order books metadata.<hr>**Response Description:**<br><br>1) **Taker and maker fees** are in percentage.<br>2) **Min base amount:** The amount of base token that can be traded in a single order.<br>3) **Min quote amount:** The amount of quote token that can be traded in a single order.<br>4) **Supported size decimals:** The number of decimal places that can be used for the size of the order.<br>5) **Supported price decimals:** The number of decimal places that can be used for the price of the order.<br>6) **Supported quote decimals:** Size Decimals + Quote Decimals.
    pub async fn order_books(&self, market_id: Option<i32>) -> Result<OrderBooks> {
        let resp = apis::order_api::order_books(&self.config, market_id)
            .await
            .inspect_err(|e| tracing::error!("unable to call `order_books`: {e}"))?;

        Ok(resp)
    }

    /// Get recent trades
    pub async fn recent_trades(&self, market_id: i32, limit: i64) -> Result<Trades> {
        let resp = apis::order_api::recent_trades(&self.config, market_id, limit)
            .await
            .inspect_err(|e| tracing::error!("unable to call `recent_trades`: {e}"))?;

        Ok(resp)
    }

    /// Get trades
    pub async fn trades(
        &self,
        sort_by: TradesSortBy,
        limit: i64,
        authorization: Option<&str>,
        auth: Option<&str>,
        market_id: Option<i32>,
        account_index: Option<i64>,
        order_index: Option<i64>,
        sort_dir: Option<TradesSortDir>,
        cursor: Option<&str>,
        from: Option<i64>,
        ask_filter: Option<i32>,
    ) -> Result<Trades> {
        let resp = apis::order_api::trades(
            &self.config,
            &sort_by.to_string(),
            limit,
            authorization,
            auth,
            market_id,
            account_index,
            order_index,
            sort_dir.map(|v| v.to_string()).as_deref(),
            cursor,
            from,
            ask_filter,
        )
        .await
        .inspect_err(|e| tracing::error!("unable to call `trades`: {e}"))?;

        Ok(resp)
    }
}
