use serenity::client::Context;
use serenity::model::Message;

mod broadcast_typing;
mod image;
mod ping;
mod prefix;
mod uptime;

pub fn handle(ctx: Context, msg: &Message, cmd: &str) {
    let (cmd, args) = match cmd.find(' ') {
        Some(n) => cmd.split_at(n),
        None => (cmd, ""),
    };
    let args = args.trim();
    match cmd.trim() {
        broadcast_typing::PREFIX => broadcast_typing::handle(ctx, msg, args),
        image::PREFIX => image::handle(ctx, msg, args),
        ping::PREFIX => ping::handle(ctx, msg, args),
        prefix::PREFIX => prefix::handle(ctx, msg, args),
        uptime::PREFIX => uptime::handle(ctx, msg, args),
        _ => image::handle(ctx, msg, cmd),
    }
}
