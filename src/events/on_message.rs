use commands;

use mysql;
use serenity::client::Context;
use serenity::model::Message;

pub fn handle(ctx: Context, msg: Message) -> ::Result<()> {
    let (db, bot_mention) = {
        let data = match ctx.data.lock() {
            Ok(data) => data,
            Err(_) => bail!("Mutex is poisoned"),
        };
        let db = match data.get::<::DbPool>() {
            Some(db) => db.clone(),
            None => bail!("No database!"),
        };
        let bot_mention = data.get::<::BotId>().map(|id| format!("<@{}>", id.0));
        (db, bot_mention)
    };

    let user: Option<String> = match db.prep_exec("SELECT (`prefix`) FROM users WHERE `id` = ?",
                                                  (msg.author.id.0,))?
              .next() {
        Some(prefix) => Some(mysql::from_row(prefix?)),
        None => None,
    };
    let mut prefixes = vec![&::CONFIG.prefix];
    if let Some(prefix) = bot_mention.as_ref() {
        prefixes.push(prefix);
    }
    if let Some(prefix) = user.as_ref() {
        prefixes.push(&prefix);
    }

    let cmd = {
        let cmd = prefixes
            .iter()
            .find(|&&prefix| msg.content.starts_with(prefix));
        if let Some(prefix) = cmd {
            msg.content.split_at(prefix.len()).1
        } else {
            return Ok(());
        }
    };

    match commands::handle(ctx, &msg, cmd.trim()) {
        Ok(response) => {
            match response {
                Some(response) => {
                    db.prep_exec(r#"INSERT INTO commands (`message_id`, `channel_id`, `response_id`)
                                  VALUES (:id, :channel, :response)
                 "#,
                           (msg.id.0, msg.channel_id.0, response.0))?;
                }
                None => (),
            };
            Ok(())
        }
        Err(e) => {
            msg.reply("An internal error occured. Sorry!")?;
            Err(e)
        }
    }
}
