#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate serenity;

mod commands;
mod config;
mod error;
mod events;

use config::Config;
use error::*;

use std::sync::RwLock;
use std::time::SystemTime;

use serenity::Client;
use serenity::model::UserId;

lazy_static! {
    static ref CONFIG: RwLock<Config> =
        RwLock::new(Config::from_file("config.json").expect("Error reading config"));
    static ref USER_ID: RwLock<Option<UserId>> = RwLock::new(None);
    static ref START_TIME: SystemTime = SystemTime::now();
}

fn main() {
    std::process::exit(match actual_main() {
                           Ok(_) => 0,
                           Err(err) => panic!("Error in main: {}", err),
                       });
}

fn actual_main() -> Result<()> {
    lazy_static::initialize(&START_TIME);

    let token = {
        let config = &CONFIG.read().unwrap();
        config.token.clone()
    };

    let mut client = Client::login(&token);
    client.on_ready(events::on_ready);
    client.on_message(events::on_message);

    Ok(client.start()?)
}
