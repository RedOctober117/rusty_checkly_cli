use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::AlertChannel;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Check<T>
where
    T: AlertChannel,
{
    alert_settings: Option<AlertEscalation>,
    retry_strategy: Option<RetryStrategy>,
    alert_channels: Option<Vec<T>>,
    name: String,
    locations: Option<Vec<Region>>,
    private_locations: Option<Vec<Region>>,
    environment_variables: Option<Vec<EnvironmentVariable>>,
    tags: Option<Vec<String>>,
    group_id: Option<String>,
    runtime_id: Option<String>,
    frequency: Option<u16>,
    frequency_offset: Option<u8>,
    activated: Option<bool>,
    muted: Option<bool>,
    double_check: Option<bool>,
    should_fail: Option<bool>,
    // test_only: Option<bool>,
    use_global_alert_settings: Option<bool>,
    run_parallel: Option<bool>,
}

impl<T> Check<T>
where
    T: AlertChannel,
{
    pub fn new_with_defaults(name: String, locations: Vec<Region>, alert_channels: Vec<T>) -> Self {
        Self {
            name,
            activated: Some(true),
            muted: Some(false),
            double_check: Some(true),
            should_fail: Some(false),
            locations: Some(locations),
            tags: None,
            alert_settings: Some(AlertEscalation::default()),
            use_global_alert_settings: Some(false),
            group_id: None,
            runtime_id: None,
            alert_channels: Some(alert_channels),
            retry_strategy: Some(RetryStrategy::default()),
            run_parallel: Some(false),
            frequency: Some(10),
            frequency_offset: Some(1),
            environment_variables: None,
            private_locations: None,
            // test_only: ,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EnvironmentVariable {
    key: String,
    value: String,
    locked: Option<bool>,
    secret: Option<bool>,
}

impl EnvironmentVariable {
    pub fn new(key: String, value: String) -> Self {
        Self {
            key,
            value,
            locked: Some(false),
            secret: Some(false),
        }
    }

    pub fn new_secret(key: String, value: String) -> Self {
        Self {
            key,
            value,
            locked: Some(false),
            secret: Some(true),
        }
    }
}

#[derive(Deserialize, Clone, Debug)]
pub enum Region {
    UsEast1,
    UsEast2,
    UsWest1,
    UsWest2,
    CaCentral1,
    SaEast1,
    EuWest1,
    EuCentral1,
    EuWest2,
    EuWest3,
    EuNorth1,
    EuSouth1,
    MeSouth1,
    ApSoutheast1,
    ApNortheast1,
    ApEast1,
    ApSoutheast2,
    ApSoutheast3,
    ApNortheast2,
    ApNortheast3,
    ApSouth1,
    AfSouth1,
}

impl Display for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            Region::UsEast1 => "us-east-1",
            Region::UsEast2 => "us-east-2",
            Region::UsWest1 => "us-west-1",
            Region::UsWest2 => "us-west-2",
            Region::CaCentral1 => "ca-central-1",
            Region::SaEast1 => "sa-east-1",
            Region::EuWest1 => "eu-west-1",
            Region::EuCentral1 => "eu-central-1",
            Region::EuWest2 => "eu-west-2",
            Region::EuWest3 => "eu-west-3",
            Region::EuNorth1 => "eu-north-1",
            Region::EuSouth1 => "eu-south-1",
            Region::MeSouth1 => "me-south-1",
            Region::ApSoutheast1 => "ap-southeast-1",
            Region::ApNortheast1 => "ap-northeast-1",
            Region::ApEast1 => "ap-east-1",
            Region::ApSoutheast2 => "ap-southeast-2",
            Region::ApSoutheast3 => "ap-southeast-3",
            Region::ApNortheast2 => "ap-northeast-2",
            Region::ApNortheast3 => "ap-northeast-3",
            Region::ApSouth1 => "ap-south-1",
            Region::AfSouth1 => "af-south-1",
        };

        write!(f, "{}", string)
    }
}

impl serde::Serialize for Region {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum RetryStrategyType {
    Fixed,
    Linear,
    Exponential,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RetryStrategy {
    base_backoff_seconds: Option<u16>,
    max_retries: Option<u8>,
    max_duration_seconds: Option<u16>,
    type_: RetryStrategyType,
    same_region: Option<bool>,
}

impl Default for RetryStrategy {
    fn default() -> Self {
        Self {
            type_: RetryStrategyType::Linear,
            base_backoff_seconds: Some(60),
            max_retries: Some(2),
            max_duration_seconds: Some(600),
            same_region: Some(true),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum AlertEscalationType {
    RunBased,
    TimeBased,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AlertEscalationReminder {
    amount: Option<u16>,
    interval: Option<u8>,
}

impl Default for AlertEscalationReminder {
    fn default() -> Self {
        Self {
            amount: Some(0),
            interval: Some(5),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AlertEscalationParallelRunFailureThreshold {
    enabled: Option<bool>,
    percentage: Option<u8>,
}

impl Default for AlertEscalationParallelRunFailureThreshold {
    fn default() -> Self {
        Self {
            enabled: Some(false),
            percentage: Some(10),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AlertEscalation {
    reminders: AlertEscalationReminder,
    parallel_run_failure_threshold: Option<AlertEscalationParallelRunFailureThreshold>,
    escalation_type: AlertEscalationType,
    run_based_escalation: u8,
    time_based_escalation: u8,
}

impl Default for AlertEscalation {
    fn default() -> Self {
        Self {
            escalation_type: AlertEscalationType::RunBased,
            reminders: AlertEscalationReminder::default(),
            run_based_escalation: 1,
            time_based_escalation: 5,
            parallel_run_failure_threshold: Some(
                AlertEscalationParallelRunFailureThreshold::default(),
            ),
        }
    }
}
