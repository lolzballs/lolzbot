use serenity::client::Context;
use serenity::model::channel::Message;

pub const PREFIX: &'static str = "type";

pub fn handle(ctx: &Context, msg: &Message, _: &str) -> super::CommandResult {
    msg.channel_id.broadcast_typing(&ctx.http)?;
    Ok((None, None))
}
