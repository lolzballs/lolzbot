use serenity::client::Context;
use serenity::model::Message;

mod broadcast_typing;
mod image;
mod ping;
mod prefix;

pub fn handle(ctx: Context, msg: &Message, cmd: &str) {
    let (cmd, args) = match cmd.find(' ') {
        Some(n) => cmd.split_at(n),
        None => (cmd, ""),
    };
    match cmd.trim() {
        broadcast_typing::PREFIX => broadcast_typing::handle(ctx, msg, args.trim()),
        image::PREFIX => image::handle(ctx, msg, args.trim()),
        ping::PREFIX => ping::handle(ctx, msg, args.trim()),
        prefix::PREFIX => prefix::handle(ctx, msg, args.trim()),
        _ => image::handle(ctx, msg, cmd),
    }
}
