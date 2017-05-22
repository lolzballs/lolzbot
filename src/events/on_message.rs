use commands;

use mysql;
use serenity::client::Context;
use serenity::model::Message;

pub fn handle(ctx: Context, msg: Message) {
    let db = ctx.data
        .lock()
        .unwrap()
        .get::<::DbPool>()
        .unwrap()
        .clone();
    let prefixes = {
        let data = ctx.data.lock().unwrap();
        let default = data.get::<::Prefix>().unwrap().clone();
        let bot_mention = data.get::<::BotId>().map(|id| format!("<@{}>", id.0));

        let user: Option<String> = db.prep_exec("SELECT (`prefix`) FROM users WHERE `id` = ?",
                                                (msg.author.id.0,))
            .unwrap()
            .next()
            .map(|r| mysql::from_row(r.unwrap()));
        let mut prefixes = vec![default];
        if let Some(prefix) = bot_mention {
            prefixes.push(prefix);
        }
        if let Some(prefix) = user {
            prefixes.push(prefix);
        }
        prefixes
    };

    let cmd = {
        let cmd = prefixes
            .iter()
            .find(|&prefix| msg.content.starts_with(prefix));
        if let Some(prefix) = cmd {
            msg.content.split_at(prefix.len()).1
        } else {
            return;
        }
    };

    match commands::handle(ctx, &msg, cmd.trim()) {
        Ok(response) => {
            match response {
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
        Err(e) => {
            msg.reply("An internal error occured. Sorry!").unwrap();
            println!("{}", e);
        }
    }

}
