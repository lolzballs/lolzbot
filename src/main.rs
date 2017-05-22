#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
#[macro_use]
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

use std::time::Instant;

use mysql::{Opts, OptsBuilder};
use serenity::Client;
use serenity::model::UserId;
use typemap::Key;

struct DbPool;
impl Key for DbPool {
    type Value = mysql::Pool;
}
struct StartTime;
impl Key for StartTime {
    type Value = Instant;
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

    let pool = {
        let mut builder = OptsBuilder::new();
        builder
            .ip_or_hostname(config.db_ip)
            .tcp_port(config.db_port.unwrap_or(3306))
            .user(config.db_user)
            .pass(config.db_pass)
            .db_name(config.db_name);

        let opts: Opts = builder.into();
        mysql::Pool::new(opts)?
    };

    {
        let mut data = client.data.lock().unwrap();
        data.insert::<DbPool>(pool);
        data.insert::<Prefix>(config.prefix);
        data.insert::<StartTime>(Instant::now());
    }

    client.on_message_delete(events::on_message_delete);
    client.on_ready(events::on_ready);
    client.on_message(events::on_message);

    Ok(client.start()?)
}
