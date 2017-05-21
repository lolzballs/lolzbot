use serenity::client::Context;
use serenity::model::Message;

pub const PREFIX: &'static str = "type";

pub fn handle(ctx: Context, msg: &Message, cmd: &str) {
    msg.channel_id.broadcast_typing().unwrap();
}
