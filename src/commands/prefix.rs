use serenity::client::Context;
use serenity::model::{Message, MessageId};

pub const PREFIX: &'static str = "prefix";

pub fn handle(ctx: Context, msg: &Message, cmd: &str) -> ::Result<Option<MessageId>> {
    let db = {
        let data = ctx.data.lock().unwrap();
        match data.get::<::DbPool>() {
            Some(db) => db.clone(),
            None => bail!("No database!"),
        }
    };
    db.prep_exec(r#"INSERT INTO `users` (`id`, `prefix`)
                    VALUES (:id, :prefix)
                    ON DUPLICATE KEY UPDATE `prefix` = :prefix"#,
                   params!{"id" => msg.author.id.0, "prefix" => cmd})?;
    Ok(Some(msg.reply(&["Prefix changed to ", cmd, "!"].concat())?
                .id))
}
