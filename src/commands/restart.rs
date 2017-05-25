use std;

use serenity::client::Context;
use serenity::model::{Message, MessageId};

pub const PREFIX: &'static str = "restart";

pub fn handle(_: Context, msg: &Message, _: &str) -> ::Result<Option<MessageId>> {
    match ::CONFIG.admins.iter().find(|&&a| a == msg.author.id.0) {
        Some(_) => std::process::exit(0),
        None => Ok(None),
    }
}
