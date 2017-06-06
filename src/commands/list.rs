use mysql;
use serenity::client::Context;
use serenity::model::{Message, MessageId};

pub const PREFIX: &'static str = "list";

pub fn list(db: &mysql::Pool, page: usize) -> ::Result<String> {
    let mut names = Vec::new();
    names.push(String::from("```"));
    names.push(String::from(format!("Page {}", page + 1)));
    for row in db.prep_exec("SELECT (`name`) FROM images LIMIT ? OFFSET ?",
                            (2, page * 2))? {
        let name: String = mysql::from_row(row?);
        names.push(name);
    }
    names.push(String::from("```"));
    Ok(names.join("\n"))
}

pub fn handle(ctx: Context, msg: &Message, _: &str) -> super::CommandResult {
    let db = get_database!(ctx);
    let message = msg.reply(&list(&db, 0)?)?;
    message.react("%E2%AC%85")?;
    message.react("%E2%9E%A1")?;
    let msg_id = message.id.0;
    let cmd_id = msg.id.0;
    Ok((Some(message.id),
        Some(Box::new(move |db| {
                          db.prep_exec("INSERT INTO paginations (`message_id`, `command_id`) VALUES (?, ?)",
                   (msg_id, cmd_id))?;
                          Ok(())
                      }))))
}