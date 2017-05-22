use serenity::client::Context;
use serenity::model::Message;

mod on_message;
mod on_message_delete;
mod on_ready;

pub use self::on_ready::handle as on_ready;
pub use self::on_message_delete::handle as on_message_delete;

pub fn on_message(ctx: Context, msg: Message) {
    let owner = {
        let data = ctx.data.lock().unwrap();
        data.get::<::OwnerId>().map(|i| i.clone())
    };
    match self::on_message::handle(ctx, msg) {
        Ok(_) => (),
        Err(e) => {
            match owner {
                Some(owner) => {
                    owner
                        .get()
                        .and_then(|o| o.dm(&format!("{:?}", e)))
                        .unwrap();
                }
                None => (),
            }
        }
    }
}
