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

pub trait AlertChannel {
    fn get_channel_properties(&self) -> &AlertChannelProperties;
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
