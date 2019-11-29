use commands;

use mysql;
use serenity::client::Context;
use serenity::model::channel::Message;

pub fn handle(ctx: Context, msg: Message) -> ::Result<()> {
    let (db, bot_mention) = {
        let mut data = ctx.data.write();
        let db = match data.get_mut::<::DbPool>() {
            Some(db) => db.clone(),
            None => bail!("Could not get ::DbPool"),
        };
        let bot_mention = data.get::<::BotId>().map(|id| format!("<@{}>", id.0));
        (db, bot_mention)
    };

    let user: Option<String> = match db
        .prep_exec(
            "SELECT (`prefix`) FROM users WHERE `id` = ?",
            (msg.author.id.0,),
        )?
        .next()
    {
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

    match commands::handle(&ctx, &msg, cmd.trim()) {
        Ok((response, cb)) => {
            match response {
                Some(response) => {
                    db.prep_exec(
                        r#"INSERT INTO commands (`message_id`, `channel_id`, `response_id`)
                                  VALUES (:id, :channel, :response)
                 "#,
                        (msg.id.0, msg.channel_id.0, response.0),
                    )?;
                }
                None => (),
            };
            match cb {
                Some(f) => f(db)?,
                None => (),
            };
            Ok(())
        }
        Err(e) => {
            msg.reply(ctx.http, "An internal error occured. Sorry!")?;
            Err(e)
        }
    }
}
