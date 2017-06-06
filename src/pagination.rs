use commands;

use mysql;
use serenity::client::Context;
use serenity::model::{Reaction, ReactionType};

pub fn on_reaction(ctx: Context, r: Reaction) -> ::Result<()> {
    let not_bot = match match ctx.data.lock() {
                  Ok(data) => data,
                  Err(_) => bail!(::ErrorKind::MutexPosioned),
              }
              .get::<::BotId>() {
        Some(id) => id.0 != r.user_id.0,
        None => bail!(::ErrorKind::NoBotId),
    };
    if not_bot {
        match r.emoji {
            ReactionType::Unicode(s) => {
                let db = get_database!(ctx);
                let rows = if s == "➡" {
                    db.prep_exec("UPDATE paginations SET current_page = current_page + 1 WHERE message_id = ?", (r.message_id.0,))?
                } else if s == "⬅" {
                    db.prep_exec("UPDATE paginations SET current_page = current_page - 1 WHERE message_id = ? AND current_page > 0", (r.message_id.0,))?
                } else {
                    return Ok(());
                }.affected_rows();

                if rows == 0 {
                    return Ok(());
                } else if rows != 1 {
                    bail!("What the fuck just happened which the multiple messages???");
                }

                let row = db.prep_exec("SELECT (`current_page`) FROM paginations WHERE `message_id` = ?",
                                       (r.message_id.0,))?
                    .next();
                if let Some(row) = row {
                    let page: usize = mysql::from_row(row?);
                    let list = commands::list::list(&db, page)?;
                    r.channel_id
                        .edit_message(r.message_id, |m| m.content(&list))?;
                }
            }
            _ => (),
        }
    }
    Ok(())
}
