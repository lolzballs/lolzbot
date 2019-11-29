use serenity::client::Context;
use serenity::model::channel::Message;

pub const PREFIX: &'static str = "ping";

pub fn handle(ctx: &Context, msg: &Message, _: &str) -> super::CommandResult {
    Ok((Some(msg.reply(ctx, "Pong!")?.id), None))
}
