use mysql;
use serenity::client::Context;
use serenity::model::{channel::Message, id::MessageId};

mod broadcast_typing;
mod copy;
mod exception;
mod id;
mod image;
mod list;
mod ping;
mod prefix;
mod rename;
mod restart;
mod upload;
mod uptime;

type CommandResult = ::Result<(
    Option<MessageId>,
    Option<Box<dyn Fn(mysql::Pool) -> ::Result<()>>>,
)>;

pub fn handle(ctx: &Context, msg: &Message, orig_cmd: &str) -> CommandResult {
    let (cmd, args) = match orig_cmd.find(' ') {
        Some(n) => orig_cmd.split_at(n),
        None => (orig_cmd, ""),
    };
    let args = args.trim();
    match cmd.trim() {
        broadcast_typing::PREFIX => broadcast_typing::handle(ctx, msg, args),
        copy::PREFIX => copy::handle(ctx, msg, args),
        exception::PREFIX => exception::handle(ctx, msg, args),
        id::PREFIX => id::handle(ctx, msg, args),
        image::PREFIX => image::handle(ctx, msg, args),
        list::PREFIX => list::handle(ctx, msg, args),
        ping::PREFIX => ping::handle(ctx, msg, args),
        prefix::PREFIX => prefix::handle(ctx, msg, args),
        rename::PREFIX => rename::handle(ctx, msg, args),
        restart::PREFIX => restart::handle(ctx, msg, args),
        upload::PREFIX => upload::handle(ctx, msg, args),
        uptime::PREFIX => uptime::handle(ctx, msg, args),
        _ => image::handle(ctx, msg, orig_cmd),
    }
}
