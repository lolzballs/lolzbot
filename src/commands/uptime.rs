use std::time::SystemTime;

use serenity::client::Context;
use serenity::model::Message;

pub const PREFIX: &'static str = "uptime";

pub fn handle(_: Context, msg: &Message, _: &str) {
    let duration = SystemTime::now().duration_since(*::START_TIME).unwrap();
    msg.reply(&format!("This bot has been up for {} seconds!", duration.as_secs()))
        .unwrap();
}
