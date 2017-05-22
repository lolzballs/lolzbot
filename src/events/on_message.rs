use commands;

use serenity::client::Context;
use serenity::model::Message;

pub fn handle(ctx: Context, msg: Message) {
    let cmd = {
        let data = ctx.data.lock().unwrap();
        let bot_mention = data.get::<::BotId>().map(|id| format!("<@{}>", id.0));
        let prefix = data.get::<::Prefix>().unwrap();
        if msg.content.starts_with(prefix) {
            msg.content.split_at(prefix.len()).1
        } else if let Some(prefix) = bot_mention {
            if msg.content.starts_with(&prefix) {
                msg.content.split_at(prefix.len()).1
            } else {
                return;
            }
        } else {
            return;
        }
    };
    let db = {
        ctx.data
            .lock()
            .unwrap()
            .get::<::DbPool>()
            .unwrap()
            .clone()
    };
    match commands::handle(ctx, &msg, cmd.trim()) {
        Some(response) => {
            db.prep_exec(r#"INSERT INTO commands (`message_id`, `channel_id`, `response_id`)
                                  VALUES (:id, :channel, :response)
                 "#,
                           (msg.id.0, msg.channel_id.0, response.0))
                .unwrap();
        }
        None => (),
    }
}
