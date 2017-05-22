use serenity::client::Context;
use serenity::model::Message;

mod on_message;
mod on_message_delete;
use serenity::model::UserId;

mod on_ready;

pub use self::on_ready::handle as on_ready;
pub use self::on_message_delete::handle as on_message_delete;

pub fn on_message(ctx: Context, msg: Message) {
    match self::on_message::handle(ctx, msg) {
        Ok(_) => (),
        Err(e) => {
            UserId(::CONFIG.owner)
                .get()
                .and_then(|o| o.dm(&format!("{:?}", e)))
                .unwrap();
        }
    }
}
