use cli::email_alert_channel::EmailAlertChannel;

// https://api-test.checklyhq.com/#/

fn main() {
    let alert_channel = EmailAlertChannel::new("test_id".into(), "telemakos@telemakos.io".into());
    let email_str = serde_json::to_string(&alert_channel).unwrap();
    println!("{}", serde_json::to_string_pretty(&alert_channel).unwrap());

    // let check = Check::new_with_defaults(
    //     "test".into(),
    //     vec![Region::UsEast1],
    //     vec![EmailAlertChannel::new(
    //         "test_channel".into(),
    //         "telemakos@telemakos.io".into(),
    //     )],
    // );

    // println!("{}", serde_json::to_string_pretty(&check).unwrap());
}
