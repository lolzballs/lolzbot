use serenity::client::Context;
use serenity::model::Message;

pub const PREFIX: &'static str = "prefix";

pub fn handle(ctx: Context, msg: &Message, cmd: &str) {
    ::CONFIG.write().unwrap().prefix = cmd.to_owned();
    msg.reply(&["Prefix changed to ", cmd, "!"].join(""))
        .unwrap();
}
