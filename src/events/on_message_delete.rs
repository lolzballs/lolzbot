use mysql;
use serenity::client::Context;
use serenity::model::{ChannelId, MessageId};

pub fn handle(ctx: Context, channel: ChannelId, msg: MessageId) {
    let db = {
        ctx.data
            .lock()
            .unwrap()
            .get::<::DbPool>()
            .unwrap()
            .clone()
    };
    let res = db.prep_exec(r#"SELECT (`response_id`)
                                  FROM commands
                                  WHERE `message_id` = ?
                 "#,
                           (msg.0,))
        .unwrap();
    for row in res {
        let id: u64 = mysql::from_row(row.unwrap());
        channel.delete_message(id).unwrap();
        db.prep_exec("DELETE FROM commands WHERE `message_id` = ?", (msg.0,))
            .unwrap();
    }
}
