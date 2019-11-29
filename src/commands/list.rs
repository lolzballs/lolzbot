use mysql;
use serenity::client::Context;
use serenity::model::channel::Message;

pub const PREFIX: &'static str = "list";

pub fn list(db: &mysql::Pool, page: usize) -> ::Result<String> {
    let mut names = Vec::new();
    names.push(String::from("```"));
    names.push(String::from(format!("Page {}", page + 1)));
    for row in db.prep_exec(
        "SELECT (`name`) FROM images LIMIT ? OFFSET ?",
        (10, page * 10),
    )? {
        let name: String = mysql::from_row(row?);
        names.push(name);
    }
    names.push(String::from("```"));
    Ok(names.join("\n"))
}

pub fn handle(ctx: &Context, msg: &Message, _: &str) -> super::CommandResult {
    let db = get_data!(ctx, ::DbPool);
    let message = msg.reply(ctx, &list(&db, 0)?)?;
    message.react(ctx, "%E2%AC%85")?;
    message.react(ctx, "%E2%9E%A1")?;
    let msg_id = message.id.0;
    let cmd_id = msg.id.0;
    let author_id = msg.author.id.0;
    Ok((
        Some(message.id),
        Some(Box::new(move |db| {
            db.prep_exec("INSERT INTO paginations (`message_id`, `command_id`, `author_id`) VALUES (?, ?, ?)",
                   (msg_id, cmd_id, author_id))?;
            Ok(())
        })),
    ))
}
