use std;

use serenity::client::Context;
use serenity::model::Message;

pub const PREFIX: &'static str = "restart";

pub fn handle(ctx: Context, msg: &Message, cmd: &str) {
    std::process::exit(0);
}
