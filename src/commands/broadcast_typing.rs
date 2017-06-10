use serenity::client::Context;
use serenity::model::Message;

pub const PREFIX: &'static str = "type";

pub fn handle(_: Context, msg: &Message, _: &str) -> super::CommandResult {
    msg.channel_id.broadcast_typing()?;
    Ok((None, None))
}
