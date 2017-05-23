use mysql;
use serenity::client::Context;
use serenity::model::{ChannelId, MessageId};

pub fn handle(ctx: Context, channel: ChannelId, msg: MessageId) -> ::Result<()> {
    let db = {
        let data = match ctx.data.lock() {
            Ok(data) => data,
            Err(_) => bail!("Mutex is poisoned"),
        };
        match data.get::<::DbPool>() {
            Some(db) => db.clone(),
            None => bail!("No database!"),
        }
    };
    let res = db.prep_exec(r#"SELECT (`response_id`)
                                  FROM commands
                                  WHERE `message_id` = ?
                 "#,
                           (msg.0,))?;
    for row in res {
        let id: u64 = mysql::from_row(row?);
        channel.delete_message(id)?;
        db.prep_exec("DELETE FROM commands WHERE `message_id` = ?", (msg.0,))?;
    }
    Ok(())
}
