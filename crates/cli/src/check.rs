use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::alert_channel::AlertChannel;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Check {
    alert_settings: Option<AlertEscalation>,
    retry_strategy: Option<RetryStrategy>,
    alert_channel_subscriptions: Option<Vec<String>>,
    name: String,
    locations: Option<Vec<Region>>, // might need to change to string rq
    private_locations: Option<Vec<String>>, // might need to change to string rq
    environment_variables: Option<Vec<EnvironmentVariable>>,
    tags: Option<Vec<String>>,
    group_id: Option<String>,
    runtime_id: Option<String>,
    tear_down_snippet_id: Option<String>,
    setup_snippet_id: Option<String>,
    local_setup_script: Option<String>,
    local_tear_down_script: Option<String>,
    frequency: Option<u16>,
    frequency_offset: Option<u8>,
    degraded_response_time: Option<u16>,
    max_response_time: Option<u16>,
    activated: Option<bool>,
    muted: Option<bool>,
    double_check: Option<bool>,
    should_fail: Option<bool>,
    // test_only: Option<bool>,
    use_global_alert_settings: Option<bool>,
    run_parallel: Option<bool>,
}

impl Check {
    // pub fn new_with_defaults(
    //     name: String,
    //     locations: Vec<String>,
    //     alert_channels: Vec<String>,
    // ) -> Self {
    //     Self {
    //         name,
    //         activated: Some(true),
    //         muted: Some(false),
    //         double_check: Some(true),
    //         should_fail: Some(false),
    //         locations: Some(locations),
    //         tags: None,
    //         alert_settings: Some(AlertEscalation::default()),
    //         use_global_alert_settings: Some(false),
    //         group_id: None,
    //         runtime_id: None,
    //         alert_channel_subscriptions: Some(alert_channels),
    //         retry_strategy: Some(RetryStrategy::default()),
    //         run_parallel: Some(false),
    //         frequency: Some(10),
    //         frequency_offset: Some(1),
    //         environment_variables: None,
    //         private_locations: None,
    //         tear_down_snippet_id: None,
    //         setup_snippet_id: None,
    //         local_setup_script: None,
    //         local_tear_down_script: None,
    //         degraded_response_time: Some(10000),
    //         max_response_time: Some(20000),
    //         // test_only: ,
    //     }
    // }
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
#[serde(rename_all = "kebab-case")]
pub enum Region {
    UsEast_1,
    UsEast_2,
    UsWest_1,
    UsWest_2,
    CaCentral_1,
    SaEast_1,
    EuWest_1,
    EuCentral_1,
    EuWest_2,
    EuWest_3,
    EuNorth_1,
    EuSouth_1,
    MeSouth_1,
    ApSoutheast_1,
    ApNortheast_1,
    ApEast_1,
    ApSoutheast_2,
    ApSoutheast_3,
    ApNortheast_2,
    ApNortheast_3,
    ApSouth_1,
    AfSouth_1,
}

// impl From<String> for Region {
//     fn from(value: String) -> Self {
//         match value.as_str() {
//             "us-east-1" => Region::UsEast_1,
//             "us-east-2" => Region::UsEast_2,
//             "us-west-1" => Region::UsWest_1,
//             "us-west-2" => Region::UsWest_2,
//             "ca-central-1" => Region::CaCentral_1,
//             "sa-east-1" => Region::SaEast_1,
//             "eu-west-1" => Region::EuWest_1,
//             "eu-central-1" => Region::EuCentral_1,
//             "eu-west-2" => Region::EuWest_2,
//             "eu-west-3" => Region::EuWest_3,
//             "eu-north-1" => Region::EuNorth_1,
//             "eu-south-1" => Region::EuSouth_1,
//             "me-south-1" => Region::MeSouth_1,
//             "ap-southeast-1" => Region::ApSoutheast_1,
//             "ap-northeast-1" => Region::ApNortheast_1,
//             "ap-east-1" => Region::ApEast_1,
//             "ap-southeast-2" => Region::ApSoutheast_2,
//             "ap-southeast-3" => Region::ApSoutheast_3,
//             "ap-northeast-2" => Region::ApNortheast_2,
//             "ap-northeast-3" => Region::ApNortheast_3,
//             "ap-south-1" => Region::ApSouth_1,
//             "af-south-1" => Region::AfSouth_1,
//         }
//     }
// }

impl Display for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            Region::UsEast_1 => "us-east-1",
            Region::UsEast_2 => "us-east-2",
            Region::UsWest_1 => "us-west-1",
            Region::UsWest_2 => "us-west-2",
            Region::CaCentral_1 => "ca-central-1",
            Region::SaEast_1 => "sa-east-1",
            Region::EuWest_1 => "eu-west-1",
            Region::EuCentral_1 => "eu-central-1",
            Region::EuWest_2 => "eu-west-2",
            Region::EuWest_3 => "eu-west-3",
            Region::EuNorth_1 => "eu-north-1",
            Region::EuSouth_1 => "eu-south-1",
            Region::MeSouth_1 => "me-south-1",
            Region::ApSoutheast_1 => "ap-southeast-1",
            Region::ApNortheast_1 => "ap-northeast-1",
            Region::ApEast_1 => "ap-east-1",
            Region::ApSoutheast_2 => "ap-southeast-2",
            Region::ApSoutheast_3 => "ap-southeast-3",
            Region::ApNortheast_2 => "ap-northeast-2",
            Region::ApNortheast_3 => "ap-northeast-3",
            Region::ApSouth_1 => "ap-south-1",
            Region::AfSouth_1 => "af-south-1",
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
    FIXED,
    LINEAR,
    EXPONENTIAL,
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
            type_: RetryStrategyType::LINEAR,
            base_backoff_seconds: Some(60),
            max_retries: Some(2),
            max_duration_seconds: Some(600),
            same_region: Some(true),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
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
    run_based_escalation: RunBasedEscalation, // may need to be struct
    time_based_escalation: Option<u8>,
    request: AlertEscalationRequest,
}

impl Default for AlertEscalation {
    fn default() -> Self {
        Self {
            escalation_type: AlertEscalationType::RunBased,
            reminders: AlertEscalationReminder::default(),
            run_based_escalation: RunBasedEscalation {
                failed_run_threshold: 1,
            },
            time_based_escalation: Some(5),
            parallel_run_failure_threshold: Some(
                AlertEscalationParallelRunFailureThreshold::default(),
            ),
            request: AlertEscalationRequest::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RunBasedEscalation {
    failed_run_threshold: u8,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AlertEscalationRequest {
    method: String,
    url: String,
    follow_redirects: bool,
    #[serde(alias = "skipSSL")]
    skip_ssl: bool,
    ip_family: String,
    body: String,
    body_type: String,
    headers: Vec<Header>,
    query_parameters: Vec<QueryParameter>,
    assertions: Vec<Assertion>,
    basic_auth: BasicAuth,
}

impl Default for AlertEscalationRequest {
    fn default() -> Self {
        Self {
            method: "GET".into(),
            url: "https://google.com".into(),
            follow_redirects: true,
            skip_ssl: false,
            ip_family: "IPv4".into(),
            body: "".into(),
            body_type: "NONE".into(),
            headers: Vec::new(),
            query_parameters: Vec::new(),
            assertions: Vec::new(),
            basic_auth: BasicAuth::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Header {
    key: String,
    value: String,
    locked: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct QueryParameter {
    key: String,
    value: String,
    locked: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BasicAuth {
    username: String,
    password: String,
}

impl Default for BasicAuth {
    fn default() -> Self {
        Self {
            username: "admin".into(),
            password: "1234".into(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Assertion {
    source: String,
    compression: String,
    target: u8,
}
