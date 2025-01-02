pub mod alert_channel;
pub mod check;
pub mod construct;
pub mod email_alert_channel;
pub mod slack_alert_channel;

// #[derive(Serialize, Deserialize, Clone)]
// pub struct MSTeamsAlertChannel {
//     channel_props: AlertChannelProperties,
//     alert_name: String,
//     teams_channel_name: String,
//     webhook_url: String,
//     alert_template: Option<String>,
// }

// impl AlertChannel for MSTeamsAlertChannel {
//     fn get_channel_properties(&self) -> &AlertChannelProperties {
//         &self.channel_props
//     }
// }

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
