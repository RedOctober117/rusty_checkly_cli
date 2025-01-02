use serde::{Deserialize, Serialize};

use crate::alert_channel::AlertChannelType;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConstructProperties {
    logical_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    physical_id: Option<String>,
    type_: AlertChannelType,
    #[serde(skip_serializing_if = "Option::is_none")]
    member: Option<bool>,
}

impl ConstructProperties {
    pub fn new(
        construct_type: AlertChannelType,
        logical_id: String,
        physical_id: Option<String>,
        member: Option<bool>,
    ) -> Self {
        Self {
            type_: construct_type,
            logical_id,
            physical_id,
            member,
        }
    }
}

pub trait Construct {
    fn get_construct_properties(&self) -> ConstructProperties;
}
