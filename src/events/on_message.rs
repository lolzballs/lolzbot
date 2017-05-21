use commands;

use serenity::client::Context;
use serenity::model::Message;

pub fn handle(ctx: Context, msg: Message) {
    if msg.content.starts_with(&::CONFIG.prefix) {
        let (_, cmd) = msg.content.split_at(::CONFIG.prefix.len());
        commands::handle(ctx, &msg, cmd);
    }
}
