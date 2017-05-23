use serenity::client::Context;
use serenity::model::{ChannelId, Message, MessageId, Ready, UserId};

mod on_message;
mod on_message_delete;
mod on_ready;

macro_rules! handle {
    ($x:expr) => {
        match $x {
            Ok(_) => (),
            Err(e) => {
                let res = UserId(::CONFIG.owner)
                    .get()
                    .and_then(|o| o.dm(&format!("{:?}", e)));
                match res {
                    Ok(_) => (),
                    Err(e) => println!("CRITICAL: {:?}", e),
                }
            }
        }
    }
}

pub fn on_message_delete(ctx: Context, channel: ChannelId, msg: MessageId) {
    handle!(self::on_message_delete::handle(ctx, channel, msg));
}

pub fn on_message(ctx: Context, msg: Message) {
    handle!(self::on_message::handle(ctx, msg));
}

pub fn on_ready(ctx: Context, r: Ready) {
    handle!(self::on_ready::handle(ctx, r));
}
