use crate::{
    apis::{self, configuration::Configuration},
    config::LighterConfig,
    models::ResultCode,
    Result,
};

#[derive(Debug)]
pub struct NotificationApi {
    config: apis::configuration::Configuration,
}

impl NotificationApi {
    pub fn new(config: &LighterConfig) -> Result<Self> {
        Ok(Self {
            config: Configuration::try_from(config)?,
        })
    }

    /// Ack notification
    pub async fn notification_ack(
        &self,
        notif_id: &str,
        account_index: i64,
        authorization: Option<&str>,
        auth: Option<&str>,
    ) -> Result<ResultCode> {
        let resp = apis::notification_api::notification_ack(
            &self.config,
            notif_id,
            account_index,
            authorization,
            auth,
        )
        .await
        .inspect_err(|e| {
            tracing::error!("unable to call `notification_ack`: {e}");
        })?;

        Ok(resp)
    }
}
