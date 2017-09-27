use serenity::client::{Context, EventHandler};
use serenity::model::{ChannelId, Message, MessageId, Ready, UserId};

use std::thread;

mod on_message;
mod on_message_delete;
// mod on_reaction_add;
// mod on_reaction_remove;
mod on_ready;

macro_rules! handle {
    ($x:expr) => {
        // TODO: Asyncify this
        thread::spawn(move || {
            match $x {
                Ok(_) => (),
                Err(e) => {
                    for &admin in ::CONFIG.admins.iter() {
                        let res = UserId(admin)
                            .get()
                            .and_then(|o| o.dm(|m| m.content(&format!("{:?}", e))));
                        match res {
                            Ok(_) => (),
                            Err(e) => println!("CRITICAL: {:?}", e),
                        }
                    }
                }
            }
        });
    }
}

pub struct Handler;

impl EventHandler for Handler {
    fn on_message_delete(&self, ctx: Context, channel: ChannelId, msg: MessageId) {
        handle!(self::on_message_delete::handle(ctx, channel, msg));
    }

    fn on_message(&self, ctx: Context, msg: Message) {
        handle!(self::on_message::handle(ctx, msg));
    }

    /*
    fn on_reaction_add(&self, ctx: Context, r: Reaction) {
        handle!(self::on_reaction_add::handle(ctx, r));
    }

    fn on_reaction_remove(&self, ctx: Context, r: Reaction) {
        handle!(self::on_reaction_remove::handle(ctx, r));
    }
    */

    fn on_ready(&self, ctx: Context, r: Ready) {
        handle!(self::on_ready::handle(ctx, r));
    }
}
