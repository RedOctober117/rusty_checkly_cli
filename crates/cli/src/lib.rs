pub mod check;
pub mod email_alert_channel;

use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum AlertChannelType {
    EMAIL,
    SLACK,
    WEBHOOK,
    SMS,
    PAGERDUTY,
    OPSGENIE,
    CALL,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConstructProperties {
    type_: AlertChannelType,
    logical_id: String,
    physical_id: Option<String>,
    member: bool,
}

impl ConstructProperties {
    pub fn new(
        construct_type: AlertChannelType,
        logical_id: String,
        physical_id: Option<String>,
        member: bool,
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

pub trait AlertChannel {
    fn get_channel_properties(&self) -> &AlertChannelProperties;
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AlertChannelProperties {
    send_recovery: Option<bool>,
    send_failure: Option<bool>,
    send_degraded: Option<bool>,
    ssl_expiry: Option<bool>,
    ssl_expiry_threshold: Option<usize>,
    auto_subscribe: Option<bool>,
    subscriptions: Option<Vec<usize>>,
}

impl Default for AlertChannelProperties {
    fn default() -> Self {
        Self {
            send_recovery: Some(true),
            send_failure: Some(true),
            send_degraded: Some(false),
            ssl_expiry: Some(true),
            ssl_expiry_threshold: Some(30),
            auto_subscribe: Some(false),
            subscriptions: Some(Vec::new()),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MSTeamsAlertChannel {
    alert_name: String,
    teams_channel_name: String,
    webhook_url: String,
    alert_template: Option<String>,
    channel_props: AlertChannelProperties,
}

impl AlertChannel for MSTeamsAlertChannel {
    fn get_channel_properties(&self) -> &AlertChannelProperties {
        &self.channel_props
    }
}

// #[derive(Serialize, Deserialize, Clone)]
// pub struct HearbeatCheck {
//     period: usize,
//     period_unit: TimeUnits,
//     grace_period: usize,
//     grace_period_unit: TimeUnits,
//     channel_props: AlertChannelProperties,
// }
// #[derive(Serialize, Deserialize, Clone)]
// pub enum TimeUnits {}
