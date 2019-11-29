#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate mysql;
#[macro_use]
extern crate serde_derive;

extern crate rand;
extern crate serde;
extern crate serde_json;
extern crate serenity;
extern crate typemap;

#[macro_use]
mod macros;

mod commands;
mod config;
mod error;
mod events;
// mod pagination;

use config::Config;
use error::*;
use events::Handler;

use std::time::Instant;

use mysql::{Opts, OptsBuilder};
use serenity::client::Client;
use serenity::model::id::UserId;
use typemap::Key;

lazy_static! {
    static ref CONFIG: Config = Config::from_file("config.json").unwrap();
}

struct DbPool;
impl Key for DbPool {
    type Value = mysql::Pool;
}
struct StartTime;
impl Key for StartTime {
    type Value = Instant;
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
    let mut client = Client::new(&CONFIG.token, Handler)?;

    let pool = {
        let mut builder = OptsBuilder::new();
        builder
            .ip_or_hostname(CONFIG.db_ip.clone())
            .tcp_port(CONFIG.db_port.unwrap_or(3306))
            .user(CONFIG.db_user.clone())
            .pass(CONFIG.db_pass.clone())
            .db_name(CONFIG.db_name.clone());

        let opts: Opts = builder.into();
        mysql::Pool::new(opts)?
    };

    {
        let mut data = client.data.write();
        data.insert::<DbPool>(pool);
        data.insert::<StartTime>(Instant::now());
    }

    Ok(client.start()?)
}
