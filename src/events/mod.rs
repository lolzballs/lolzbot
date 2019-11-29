use serenity::client::{Context, EventHandler};
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::id::{ChannelId, MessageId, UserId};

use std::thread;

mod on_message;
mod on_message_delete;
// mod on_reaction_add;
// mod on_reaction_remove;
mod on_ready;

macro_rules! handle {
    ($ctx:ident, $x:expr) => {
        // TODO: Asyncify this
        let ctx = $ctx.clone();
        thread::spawn(move || match $x {
            Ok(_) => (),
            Err(e) => {
                for &admin in ::CONFIG.admins.iter() {
                    let res = UserId(admin).create_dm_channel(&ctx.http).and_then(|c| {
                        c.send_message(&ctx.http, |m| m.content(&format!("{:?}", e)))
                    });
                    match res {
                        Ok(_) => (),
                        Err(e) => println!("CRITICAL: {:?}", e),
                    }
                }
            }
        });
    };
}

pub struct Handler;

impl EventHandler for Handler {
    fn message_delete(&self, ctx: Context, channel: ChannelId, msg: MessageId) {
        handle!(ctx, self::on_message_delete::handle(ctx, channel, msg));
    }

    fn message(&self, ctx: Context, msg: Message) {
        handle!(ctx, self::on_message::handle(ctx, msg));
    }

    /*
    fn on_reaction_add(&self, ctx: Context, r: Reaction) {
        handle!(self::on_reaction_add::handle(ctx, r));
    }

    fn on_reaction_remove(&self, ctx: Context, r: Reaction) {
        handle!(self::on_reaction_remove::handle(ctx, r));
    }
    */

    fn ready(&self, ctx: Context, r: Ready) {
        handle!(ctx, self::on_ready::handle(ctx, r));
    }
}
