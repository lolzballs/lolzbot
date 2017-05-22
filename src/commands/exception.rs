use serenity::client::Context;
use serenity::model::{Message, MessageId};

pub const PREFIX: &'static str = "exception";

pub fn handle(_: Context, msg: &Message, _: &str) -> ::Result<Option<MessageId>> {
    if ::CONFIG.owner == msg.author.id.0 {
        bail!("You wanted an exception?");
    } else {
        Ok(None)
    }
}
