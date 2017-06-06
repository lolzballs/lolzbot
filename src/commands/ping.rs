use serenity::client::Context;
use serenity::model::{Message, MessageId};

pub const PREFIX: &'static str = "ping";

pub fn handle(_: Context, msg: &Message, _: &str) -> super::CommandResult {
    Ok((Some(msg.reply("Pong!")?.id), None))
}
