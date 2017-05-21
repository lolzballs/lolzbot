use std;

use serenity::client::Context;
use serenity::model::Message;

pub const PREFIX: &'static str = "restart";

pub fn handle(_: Context, _: &Message, _: &str) {
    std::process::exit(0);
}
