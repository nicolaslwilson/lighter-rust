use crate::{
    apis::{self, configuration::Configuration},
    config::LighterConfig,
    models::ReferralPoints,
    Result,
};

#[derive(Debug)]
pub struct ReferralApi {
    config: apis::configuration::Configuration,
}

impl ReferralApi {
    pub fn new(config: &LighterConfig) -> Result<Self> {
        Ok(Self {
            config: Configuration::try_from(config)?,
        })
    }

    /// Get referral points
    pub async fn referral_points(
        &self,
        account_index: i64,
        authorization: Option<&str>,
        auth: Option<&str>,
    ) -> Result<ReferralPoints> {
        let resp =
            apis::referral_api::referral_points(&self.config, account_index, authorization, auth)
                .await
                .inspect_err(|e| {
                    tracing::error!("unable to call `referral_points`: {e}");
                })?;

        Ok(resp)
    }
}
