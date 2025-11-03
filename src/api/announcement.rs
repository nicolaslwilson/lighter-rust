use crate::{
    apis::{self, configuration::Configuration},
    config::LighterConfig,
    models::Announcements,
    Result,
};

#[derive(Debug)]
pub struct AnnouncementApi {
    config: apis::configuration::Configuration,
}

impl AnnouncementApi {
    pub fn new(config: &LighterConfig) -> Result<Self> {
        Ok(Self {
            config: Configuration::try_from(config)?,
        })
    }

    /// Get announcement
    pub async fn announcement(&self) -> Result<Announcements> {
        let resp = apis::announcement_api::announcement(&self.config)
            .await
            .inspect_err(|e| tracing::error!("unable to call `announcement`: {e}"))?;

        Ok(resp)
    }
}
