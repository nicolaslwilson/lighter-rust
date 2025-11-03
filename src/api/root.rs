use crate::{
    apis::{self, configuration::Configuration},
    config::LighterConfig,
    models::{Status, ZkLighterInfo},
    Result,
};

#[derive(Debug)]
pub struct RootApi {
    config: apis::configuration::Configuration,
}

impl RootApi {
    pub fn new(config: &LighterConfig) -> Result<Self> {
        Ok(Self {
            config: Configuration::try_from(config)?,
        })
    }

    /// Get info of zklighter
    pub async fn info(&self) -> Result<ZkLighterInfo> {
        let resp = apis::root_api::info(&self.config)
            .await
            .inspect_err(|e| tracing::error!("unable to call `info`: {e}"))?;

        Ok(resp)
    }

    /// Get status of zklighter
    pub async fn status(&self) -> Result<Status> {
        let resp = apis::root_api::status(&self.config)
            .await
            .inspect_err(|e| tracing::error!("unable to call `status`: {e}"))?;

        Ok(resp)
    }
}
