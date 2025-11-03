use crate::{
    apis::{self, configuration::Configuration},
    config::LighterConfig,
    models::{Candlesticks, Fundings},
    Result,
};

#[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq)]
pub enum CandlestickInterval {
    #[strum(to_string = "1m")]
    OneMinute,
    #[strum(to_string = "5m")]
    FiveMinutes,
    #[strum(to_string = "15m")]
    FifteenMinutes,
    #[strum(to_string = "30")]
    ThirtyMinutes,
    #[strum(to_string = "1h")]
    OneHour,
    #[strum(to_string = "4h")]
    FourHours,
    #[strum(to_string = "1d")]
    OneDay,
    #[strum(to_string = "1w")]
    OneWeek,
}

#[derive(Debug, Clone, Copy, strum::Display, PartialEq, Eq)]
pub enum FundingInterval {
    #[strum(to_string = "1h")]
    OneHour,
    #[strum(to_string = "1d")]
    OneDay,
}

#[derive(Debug)]
pub struct CandlestickApi {
    config: apis::configuration::Configuration,
}

impl CandlestickApi {
    pub fn new(config: &LighterConfig) -> Result<Self> {
        Ok(Self {
            config: Configuration::try_from(config)?,
        })
    }

    /// Get candlesticks
    pub async fn candlesticks(
        &self,
        market_id: i32,
        resolution: CandlestickInterval,
        start_timestamp: i64,
        end_timestamp: i64,
        count_back: i64,
        set_timestamp_to_end: Option<bool>,
    ) -> Result<Candlesticks> {
        let resp = apis::candlestick_api::candlesticks(
            &self.config,
            market_id,
            &resolution.to_string(),
            start_timestamp,
            end_timestamp,
            count_back,
            set_timestamp_to_end,
        )
        .await
        .inspect_err(|e| {
            tracing::error!("unable to call `candlestick`: {e}");
        })?;

        Ok(resp)
    }

    /// Get fundings
    pub async fn fundings(
        &self,
        market_id: i32,
        resolution: FundingInterval,
        start_timestamp: i64,
        end_timestamp: i64,
        count_back: i64,
    ) -> Result<Fundings> {
        let resp = apis::candlestick_api::fundings(
            &self.config,
            market_id,
            &resolution.to_string(),
            start_timestamp,
            end_timestamp,
            count_back,
        )
        .await
        .inspect_err(|e| {
            tracing::error!("unable to call `fundings`: {e}");
        })?;

        Ok(resp)
    }
}
