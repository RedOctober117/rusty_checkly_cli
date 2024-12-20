use cli::email_alert_channel::EmailAlertChannel;

// https://api-test.checklyhq.com/#/

fn main() {
    let alert_channel = EmailAlertChannel::new("test_id".into(), "telemakos@telemakos.io".into());
    println!("{}", serde_json::to_string_pretty(&alert_channel).unwrap())
}
