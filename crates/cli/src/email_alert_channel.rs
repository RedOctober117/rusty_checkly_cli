use serde::{Deserialize, Serialize};

use crate::{
    alert_channel::{AlertChannel, AlertChannelProperties, AlertChannelType},
    construct::{Construct, ConstructProperties},
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EmailAlertChannelConfig {
    address: String,
}

impl EmailAlertChannelConfig {
    pub fn new(address: String) -> Self {
        Self { address }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EmailAlertChannel {
    #[serde(flatten)]
    construct_props: ConstructProperties,
    #[serde(flatten)]
    channel_props: AlertChannelProperties,
    config: EmailAlertChannelConfig,
}

impl EmailAlertChannel {
    pub fn new(logical_id: String, address: String) -> Self {
        Self {
            config: EmailAlertChannelConfig { address },
            channel_props: AlertChannelProperties::default(),
            construct_props: ConstructProperties::new(
                AlertChannelType::EMAIL,
                logical_id,
                None,
                Some(false),
            ),
        }
    }

    pub fn address(&self) -> &str {
        &self.config.address
    }
}

impl AlertChannel for EmailAlertChannel {
    fn get_channel_properties(&self) -> &AlertChannelProperties {
        &self.channel_props
    }
}

impl Construct for EmailAlertChannel {
    fn get_construct_properties(&self) -> ConstructProperties {
        self.construct_props.clone()
    }
}
