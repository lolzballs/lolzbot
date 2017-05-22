use serenity::client::Context;
use serenity::model::{Message, MessageId};

pub const PREFIX: &'static str = "prefix";

pub fn handle(ctx: Context, msg: &Message, cmd: &str) -> Option<MessageId> {
    let db = ctx.data
        .lock()
        .unwrap()
        .get::<::DbPool>()
        .unwrap()
        .clone();
    db.prep_exec(r#"INSERT INTO `users` (`id`, `prefix`)
                    VALUES (:id, :prefix)
                    ON DUPLICATE KEY UPDATE `prefix` = :prefix"#,
                   params!{"id" => msg.author.id.0, "prefix" => cmd})
        .unwrap();
    Some(msg.reply(&["Prefix changed to ", cmd, "!"].concat())
             .unwrap()
             .id)
}
