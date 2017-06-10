use mysql;
use serenity::client::Context;
use serenity::model::Message;

pub const PREFIX: &'static str = "copy";

pub fn handle(ctx: Context, msg: &Message, cmd: &str) -> super::CommandResult {
    match ::CONFIG.admins.iter().find(|&&a| a == msg.author.id.0) {
        Some(_) => {
            let (id, name) = match cmd.find(" ") {
                Some(s) => cmd.split_at(s),
                None => return Ok((None, None)),
            };
            let name = name.trim();

            if name.len() > 191 {
                return Ok((Some(msg.reply("Name is too long (max. 191 chars)")?.id), None));
            }

            let id = match id.trim().parse::<u64>() {
                Ok(id) => id,
                Err(_) => return Ok((None, None)),
            };

            let db = get_database!(ctx);
            let res = db.prep_exec("INSERT INTO images (`name`, `code`) SELECT ?, `code` FROM images WHERE `id` = ?", (name, id));
            match res {
                Ok(res) => {
                    if res.affected_rows() == 0 {
                        Ok((None, None))
                    } else {
                        Ok((Some(msg.reply("Image created!")?.id), None))
                    }
                }
                Err(mysql::Error::MySqlError(mysql::MySqlError { code: 1062, .. })) => {
                    Ok((Some(msg.reply("Image has duplicate name")?.id), None))
                }
                Err(e) => Err(e.into()),
            }
        }
        None => Ok((None, None)),
    }
}
