pub mod check;
pub mod email_alert_channel;
pub mod slack_alert_channel;

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

pub trait AlertChannel {
    fn get_channel_properties(&self) -> &AlertChannelProperties;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct GroupCheck {
    name: String,
    id: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct Subscription {
    check_group: GroupCheck,
    check: String,
    check_id: String,
    activated: bool,
    alert_channel_id: usize,
    group_id: usize,
    id: usize,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AlertChannelProperties {
    subscriptions: Option<Vec<Subscription>>,
    ssl_expiry_threshold: Option<u8>,
    send_recovery: Option<bool>,
    send_failure: Option<bool>,
    send_degraded: Option<bool>,
    ssl_expiry: Option<bool>,
    auto_subscribe: Option<bool>,
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
    channel_props: AlertChannelProperties,
    alert_name: String,
    teams_channel_name: String,
    webhook_url: String,
    alert_template: Option<String>,
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
