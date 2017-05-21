#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

extern crate mysql;
extern crate serde;
extern crate serde_json;
extern crate serenity;
extern crate typemap;

mod commands;
mod config;
mod error;
mod events;

use config::Config;
use error::*;

use std::time::SystemTime;

use serenity::Client;
use serenity::model::UserId;
use typemap::Key;

struct StartTime;
impl Key for StartTime {
    type Value = SystemTime;
}

struct Prefix;
impl Key for Prefix {
    type Value = String;
}

struct BotId;
impl Key for BotId {
    type Value = UserId;
}

fn main() {
    std::process::exit(match actual_main() {
                           Ok(_) => 0,
                           Err(err) => panic!("Error in main: {}", err),
                       });
}

fn actual_main() -> Result<()> {
    let config = Config::from_file("config.json")?;

    let mut client = Client::login(&config.token);
    {
        let mut data = client.data.lock().unwrap();
        data.insert::<Prefix>(config.prefix);
        data.insert::<StartTime>(SystemTime::now());
    }
    client.on_ready(events::on_ready);
    client.on_message(events::on_message);

    Ok(client.start()?)
}
