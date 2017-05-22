use serenity::client::Context;
use serenity::model::{Message, MessageId};

pub const PREFIX: &'static str = "type";

pub fn handle(_: Context, msg: &Message, _: &str) -> ::Result<Option<MessageId>> {
    msg.channel_id.broadcast_typing()?;
    Ok(None)
}
