use std;

use serenity::client::Context;
use serenity::model::{Message, MessageId};

pub const PREFIX: &'static str = "restart";

pub fn handle(_: Context, msg: &Message, _: &str) -> ::Result<Option<MessageId>> {
    if ::CONFIG.owner == msg.author.id.0 {
        std::process::exit(0);
    } else {
        Ok(None)
    }
}
