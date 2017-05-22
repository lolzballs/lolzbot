use serenity::client::Context;
use serenity::model::{Message, MessageId};

pub const PREFIX: &'static str = "prefix";

pub fn handle(ctx: Context, msg: &Message, cmd: &str) -> Option<MessageId> {
    ctx.data
        .lock()
        .unwrap()
        .insert::<::Prefix>(cmd.to_owned());
    Some(msg.reply(&["Prefix changed to ", cmd, "!"].join(""))
             .unwrap()
             .id)
}
