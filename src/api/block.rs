use crate::{
    apis::{self, configuration::Configuration},
    config::LighterConfig,
    models::{Blocks, CurrentHeight},
    Result,
};

#[derive(Debug, Clone, strum::Display)]
#[strum(serialize_all = "snake_case")]
pub enum BlockBy {
    Commitment,
    Height,
}

#[derive(Debug, Clone, strum::Display)]
#[strum(serialize_all = "snake_case")]
pub enum BlocksSort {
    Asc,
    Desc,
}

#[derive(Debug)]
pub struct BlockApi {
    config: apis::configuration::Configuration,
}

impl BlockApi {
    pub fn new(config: &LighterConfig) -> Result<Self> {
        Ok(Self {
            config: Configuration::try_from(config)?,
        })
    }

    /// Get block by its height or commitment
    pub async fn block(&self, by: BlockBy, value: &str) -> Result<Blocks> {
        let resp = apis::block_api::block(&self.config, &by.to_string(), value)
            .await
            .inspect_err(|e| tracing::error!("unable to call `block`: {e}"))?;

        Ok(resp)
    }

    /// Get blocks
    pub async fn blocks(
        &self,
        limit: i64,
        index: Option<i64>,
        sort: Option<BlocksSort>,
    ) -> Result<Blocks> {
        let resp = apis::block_api::blocks(
            &self.config,
            limit,
            index,
            sort.map(|v| v.to_string()).as_deref(),
        )
        .await
        .inspect_err(|e| tracing::error!("unable to call `blocks`: {e}"))?;

        Ok(resp)
    }

    /// Get current height
    pub async fn current_height(&self) -> Result<CurrentHeight> {
        let resp = apis::block_api::current_height(&self.config)
            .await
            .inspect_err(|e| tracing::error!("unable to call `current_height`: {e}"))?;

        Ok(resp)
    }
}
