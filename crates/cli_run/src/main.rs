use std::{fs::File, io::Read};

use cli::{email_alert_channel::EmailAlertChannel, AlertChannelProperties};
use serde::{Deserialize, Serialize};
use toml::Table;

// https://api-test.checklyhq.com/#/

fn main() {
    // let alert_channel = EmailAlertChannel::new("test_id".into(), "telemakos@telemakos.io".into());
    // let email_str = serde_json::to_string(&alert_channel).unwrap();
    // println!("{}", serde_json::to_string_pretty(&alert_channel).unwrap());

    // let check = Check::new_with_defaults(
    //     "test".into(),
    //     vec![Region::UsEast1],
    //     vec![EmailAlertChannel::new(
    //         "test_channel".into(),
    //         "telemakos@telemakos.io".into(),
    //     )],
    // );

    // #[derive(Serialize, Deserialize, Debug)]
    // #[serde(rename_all = "camelCase")]
    // struct GroupCheck {
    //     name: String,
    //     id: usize,
    // }

    // #[derive(Serialize, Deserialize, Debug)]
    // #[serde(rename_all = "camelCase")]
    // struct Subscription {
    //     check_group: Vec<GroupCheck>,
    //     check: String,
    //     check_id: String,
    //     activated: bool,
    //     alert_channel_id: usize,
    //     group_id: usize,
    //     id: usize,
    // }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    struct EmailConfig {
        address: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    struct Email {
        #[serde(alias = "type")]
        type_: String,
        // subscriptions: Vec<Subscription>,
        config: EmailConfig,
        // id: usize, potentially not needed at all
        #[serde(flatten)]
        props: AlertChannelProperties,
    }

    // println!("{}", serde_json::to_string_pretty(&check).unwrap());
    let mut buffer = String::new();
    let mut file = File::open("email-alert-channel.toml").unwrap();
    file.read_to_string(&mut buffer).unwrap();
    let test: EmailAlertChannel = toml::from_str(&buffer).unwrap();

    println!("{:?}\n\n", test);

    println!("{}", serde_json::to_string_pretty(&test).unwrap());
}
