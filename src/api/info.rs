use crate::{
    apis::{self, configuration::Configuration},
    config::LighterConfig,
    models::{RespWithdrawalDelay, TransferFeeInfo},
    Result,
};

#[derive(Debug)]
pub struct InfoApi {
    config: apis::configuration::Configuration,
}

impl InfoApi {
    pub fn new(config: &LighterConfig) -> Result<Self> {
        Ok(Self {
            config: Configuration::try_from(config)?,
        })
    }

    /// Withdrawal delay in seconds
    pub async fn transfer_fee_info(
        &self,
        account_index: i64,
        authorization: Option<&str>,
        auth: Option<&str>,
        to_account_index: Option<i64>,
    ) -> Result<TransferFeeInfo> {
        let resp = apis::info_api::transfer_fee_info(
            &self.config,
            account_index,
            authorization,
            auth,
            to_account_index,
        )
        .await
        .inspect_err(|e| {
            tracing::error!("unable to call `transfer_fee_info`: {e}");
        })?;

        Ok(resp)
    }

    pub async fn withdrawal_delay(&self) -> Result<RespWithdrawalDelay> {
        let resp = apis::info_api::withdrawal_delay(&self.config)
            .await
            .inspect_err(|e| tracing::error!("unable to call `withdrawal_delay`: {e}"))?;

        Ok(resp)
    }
}
