use serenity::client::Context;
use serenity::model::channel::Message;

pub const PREFIX: &'static str = "exception";

pub fn handle(_: &Context, msg: &Message, _: &str) -> super::CommandResult {
    match ::CONFIG.admins.iter().find(|&&a| a == msg.author.id.0) {
        Some(_) => bail!(::ErrorKind::UserError),
        None => Ok((None, None)),
    }
}
