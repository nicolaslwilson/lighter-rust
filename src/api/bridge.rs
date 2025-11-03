use crate::{
    apis::{self, configuration::Configuration},
    config::LighterConfig,
    models::RespGetFastBridgeInfo,
    Result,
};

#[derive(Debug)]
pub struct BridgeApi {
    config: apis::configuration::Configuration,
}

impl BridgeApi {
    pub fn new(config: &LighterConfig) -> Result<Self> {
        Ok(Self {
            config: Configuration::try_from(config)?,
        })
    }

    /// Get fast bridge info
    pub async fn fastbridge_info(&self) -> Result<RespGetFastBridgeInfo> {
        let resp = apis::bridge_api::fastbridge_info(&self.config)
            .await
            .inspect_err(|e| tracing::error!("unable to call `fastbridge_info: {e}"))?;

        Ok(resp)
    }
}
