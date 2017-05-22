use serenity::client::Context;
use serenity::model::{Message, MessageId};

pub const PREFIX: &'static str = "type";

pub fn handle(_: Context, msg: &Message, _: &str) -> Option<MessageId> {
    msg.channel_id.broadcast_typing().unwrap();
    None
}
