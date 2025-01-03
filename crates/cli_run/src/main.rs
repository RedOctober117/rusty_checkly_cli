use std::{fs::File, io::Read};

use cli::{
    alert_channel::AlertChannelProperties, check::Check, email_alert_channel::EmailAlertChannel,
};
use serde::{Deserialize, Serialize};

// https://api-test.checklyhq.com/#/

fn main() {
    // {
    //     let mut buffer = String::new();
    //     let mut file = File::open("email-alert-channel.toml").unwrap();
    //     file.read_to_string(&mut buffer).unwrap();
    //     let test: EmailAlertChannel = toml::from_str(&buffer).unwrap();

    //     println!("{:?}\n\n", test);

    //     println!("{}", serde_json::to_string_pretty(&test).unwrap());
    // }
    {
        let mut buffer = String::new();
        let mut file = File::open("api-check.toml").unwrap();
        file.read_to_string(&mut buffer).unwrap();
        let test: Check = toml::from_str(&buffer).unwrap();

        println!("{:?}\n\n", test);

        println!("{}", serde_json::to_string_pretty(&test).unwrap());
    }
}
