use serenity::client::Context;
use serenity::model::Message;

mod image;
mod ping;

pub fn handle(ctx: Context, msg: &Message, cmd: &str) {
    if cmd.starts_with(' ') {
        let (_, cmd) = cmd.split_at(1);
        let (cmd, args) = match cmd.find(' ') {
            Some(n) => cmd.split_at(n),
            None => (cmd, ""),
        };
        println!("{}", cmd);
        match cmd.trim() {
            image::PREFIX => image::handle(ctx, msg, args.trim()),
            ping::PREFIX => ping::handle(ctx, msg, args.trim()),
            _ => println!("{}", cmd),
        }
    } else {
        image::handle(ctx, msg, cmd);
    }
}
