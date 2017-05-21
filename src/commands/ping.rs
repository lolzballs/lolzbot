use serenity::client::Context;
use serenity::model::Message;

pub const PREFIX: &'static str = "ping";

pub fn handle(ctx: Context, msg: &Message, cmd: &str) {
    msg.reply("Pong!").unwrap();
}
