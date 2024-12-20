use serde::{Deserialize, Serialize};

use crate::AlertChannel;

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Check<T>
where
    T: AlertChannel,
{
    name: String,
    activated: Option<bool>,
    muted: Option<bool>,
    double_check: Option<bool>,
    should_fail: Option<bool>,
    runtime_id: Option<String>,
    locations: Option<Vec<Region>>,
    private_locations: Option<Vec<Region>>,
    tags: Option<Vec<String>>,
    frequency: Option<usize>,
    frequency_offset: Option<usize>,
    environment_variables: Option<Vec<EnvironmentVariable>>,
    group_id: Option<String>,
    alert_channels: Option<Vec<T>>,
    // test_only: Option<bool>,
    retry_strategy: Option<RetryStrategy>,
    alert_settings: Option<AlertEscalation>,
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

#[derive(Serialize, Deserialize, Clone, Debug)]
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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum RetryStrategyType {
    Fixed,
    Linear,
    Exponential,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RetryStrategy {
    type_: RetryStrategyType,
    base_backoff_seconds: Option<usize>,
    max_retries: Option<usize>,
    max_duration_seconds: Option<usize>,
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
    amount: Option<usize>,
    interval: Option<usize>,
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
    percentage: Option<usize>,
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
    escalation_type: AlertEscalationType,
    reminders: AlertEscalationReminder,
    run_based_escalation: usize,
    time_based_escalation: usize,
    parallel_run_failure_threshold: Option<AlertEscalationParallelRunFailureThreshold>,
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
