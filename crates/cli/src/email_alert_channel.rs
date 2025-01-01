use serde::{Deserialize, Serialize};

use crate::{
    AlertChannel, AlertChannelProperties, AlertChannelType, Construct, ConstructProperties,
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
    config: EmailAlertChannelConfig,
    #[serde(flatten)]
    channel_props: AlertChannelProperties,
    #[serde(flatten)]
    construct_props: ConstructProperties,
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
                false,
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
