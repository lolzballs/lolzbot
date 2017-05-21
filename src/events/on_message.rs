use commands;

use serenity::client::Context;
use serenity::model::Message;

pub fn handle(ctx: Context, msg: Message) {
    let bot_mention = ::USER_ID
        .read()
        .unwrap()
        .map(|id| format!("<@{}>", id.0));
    let cmd = {
        let ref prefix = ::CONFIG.read().unwrap().prefix;
        if msg.content.starts_with(prefix) {
            msg.content.split_at(prefix.len()).1
        } else if let Some(prefix) = bot_mention {
            if msg.content.starts_with(&prefix) {
                msg.content.split_at(prefix.len()).1
            } else {
                return;
            }
        } else {
            return;
        }
    };
    commands::handle(ctx, &msg, cmd.trim());
}
