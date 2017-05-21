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

use serenity::Client;

lazy_static! {
    static ref CONFIG: Config =
        Config::from_file("config.json").expect("Error reading config");
}

fn main() {
    std::process::exit(match actual_main() {
                           Ok(_) => 0,
                           Err(err) => panic!("Error in main: {}", err),
                       });
}

fn actual_main() -> Result<()> {
    let mut client = Client::login(&CONFIG.token);
    client.on_message(events::on_message);

    Ok(client.start()?)
}
