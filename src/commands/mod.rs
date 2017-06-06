use mysql;
use serenity::client::Context;
use serenity::model::{Message, MessageId};

mod broadcast_typing;
mod exception;
mod image;
pub mod list;
mod ping;
mod prefix;
mod restart;
mod upload;
mod uptime;

type CommandResult = ::Result<(Option<MessageId>, Option<Box<Fn(mysql::Pool) -> ::Result<()>>>)>;

pub fn handle(ctx: Context, msg: &Message, orig_cmd: &str) -> CommandResult {
    let (cmd, args) = match orig_cmd.find(' ') {
        Some(n) => orig_cmd.split_at(n),
        None => (orig_cmd, ""),
    };
    let args = args.trim();
    match cmd.trim() {
        broadcast_typing::PREFIX => broadcast_typing::handle(ctx, msg, args),
        exception::PREFIX => exception::handle(ctx, msg, args),
        image::PREFIX => image::handle(ctx, msg, args),
        list::PREFIX => list::handle(ctx, msg, args),
        ping::PREFIX => ping::handle(ctx, msg, args),
        prefix::PREFIX => prefix::handle(ctx, msg, args),
        restart::PREFIX => restart::handle(ctx, msg, args),
        upload::PREFIX => upload::handle(ctx, msg, args),
        uptime::PREFIX => uptime::handle(ctx, msg, args),
        _ => image::handle(ctx, msg, orig_cmd),
    }
}
