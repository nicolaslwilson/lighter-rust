use crate::{
    apis::{self, configuration::Configuration},
    config::LighterConfig,
    models::FundingRates,
    Result,
};

#[derive(Debug)]
pub struct FundingApi {
    config: apis::configuration::Configuration,
}

impl FundingApi {
    pub fn new(config: &LighterConfig) -> Result<Self> {
        Ok(Self {
            config: Configuration::try_from(config)?,
        })
    }

    /// Get funding rates
    pub async fn funding_rates(&self) -> Result<FundingRates> {
        let resp = apis::funding_api::funding_rates(&self.config)
            .await
            .inspect_err(|e| tracing::error!("unable to call `funding_rates`: {e}"))?;

        Ok(resp)
    }
}
