use serenity::client::Context;
use serenity::model::{Message, MessageId};

pub const PREFIX: &'static str = "exception";

pub fn handle(ctx: Context, msg: &Message, _: &str) -> ::Result<Option<MessageId>> {
    let owner = {
        let data = ctx.data.lock().unwrap();
        data.get::<::OwnerId>().map(|i| i.clone())
    };
    match owner {
        Some(owner) => {
            if owner == msg.author.id {
                bail!("You wanted an exception?");
            } else {
                Ok(None)
            }
        }
        None => Ok(None),
    }
}
