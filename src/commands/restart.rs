use std;

use serenity::client::Context;
use serenity::model::{Message, MessageId};

pub const PREFIX: &'static str = "restart";

pub fn handle(_: Context, _: &Message, _: &str) -> ::Result<Option<MessageId>> {
    std::process::exit(0);
}
