use serde::{ser, Deserialize, Serialize};

use crate::{
    AlertChannel, AlertChannelProperties, AlertChannelType, Construct, ConstructProperties,
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SlackAlertChannelConfig {
    url: String,
    channel: String,
}

impl SlackAlertChannelConfig {
    pub fn new(url: String, channel: String) -> Self {
        Self { url, channel }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SlackAlertChannel {
    #[serde(flatten)]
    construct_props: ConstructProperties,
    #[serde(flatten)]
    channel_props: AlertChannelProperties,
    config: SlackAlertChannelConfig,
}

impl SlackAlertChannel {
    pub fn new(logical_id: String, url: String, address: String) -> Self {
        Self {
            construct_props: ConstructProperties::new(
                AlertChannelType::SLACK,
                logical_id,
                None,
                false,
            ),
            channel_props: AlertChannelProperties::default(),
            config: 
        }
    }
}
