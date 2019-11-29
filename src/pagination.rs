use commands;

use mysql;
use serenity::client::Context;
use serenity::model::{Reaction, ReactionType};

pub fn on_reaction(ctx: Context, r: Reaction) -> ::Result<()> {
    let not_bot = match ctx.data.lock().get::<::BotId>() {
        Some(id) => id.0 != r.user_id.0,
        None => bail!(::ErrorKind::NoBotId),
    };
    if not_bot {
        match r.emoji {
            ReactionType::Unicode(s) => {
                let db = get_data!(ctx, ::DbPool);
                let (mut page, author): (usize, u64) = {
                    if let Some(row) = db.prep_exec("SELECT `current_page`, `author_id` FROM paginations WHERE `message_id` = ?",
                                       (r.message_id.0,))?.next() {
                        mysql::from_row(row?)
                    } else {
                        return Ok(());
                    }
                };
                if r.user_id.0 != author {
                    return Ok(());
                }

                let rows = if s == "➡" {
                    page += 1;
                    db.prep_exec("UPDATE paginations SET current_page = current_page + 1 WHERE message_id = ?", (r.message_id.0,))?
                } else if s == "⬅" {
                    if page > 0 {
                        page -= 1;
                    } else {
                        return Ok(());
                    }
                    db.prep_exec("UPDATE paginations SET current_page = current_page - 1 WHERE message_id = ? AND current_page > 0", (r.message_id.0,))?
                } else {
                    return Ok(());
                }.affected_rows();

                if rows == 0 {
                    return Ok(());
                } else if rows != 1 {
                    bail!("What the fuck just happened with the multiple messages???");
                }

                let list = commands::list::list(&db, page)?;
                r.channel_id
                    .edit_message(r.message_id, |m| m.content(&list))?;
            }
            _ => (),
        }
    }
    Ok(())
}
