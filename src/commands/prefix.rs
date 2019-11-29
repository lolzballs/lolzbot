use mysql;
use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::utils::MessageBuilder;

pub const PREFIX: &'static str = "prefix";

pub fn handle(ctx: &Context, msg: &Message, cmd: &str) -> super::CommandResult {
    let db = get_data!(ctx, ::DbPool);
    if cmd.len() == 0 {
        let user_prefix: Option<_> = db
            .prep_exec(
                "SELECT (`prefix`) FROM users WHERE `id` = ?",
                (msg.author.id.0,),
            )?
            .next();
        let string = {
            let mut builder = MessageBuilder::new();
            let builder = builder
                .push("You can always mention me or use one of the following...\n")
                .push("The configured global prefix is: ")
                .push_mono(&::CONFIG.prefix);

            if let Some(prefix) = user_prefix {
                let prefix: String = mysql::from_row(prefix?);
                builder
                    .push("\nYour user prefix is: ")
                    .push_mono(&prefix)
                    .build()
            } else {
                builder.build()
            }
        };

        Ok((Some(msg.reply(ctx, &string)?.id), None))
    } else {
        db.prep_exec(
            r#"INSERT INTO `users` (`id`, `prefix`)
                    VALUES (:id, :prefix)
                    ON DUPLICATE KEY UPDATE `prefix` = :prefix"#,
            params! {"id" => msg.author.id.0, "prefix" => cmd},
        )?;
        Ok((
            Some(
                msg.reply(ctx, &["Prefix changed to ", cmd, "!"].concat())?
                    .id,
            ),
            None,
        ))
    }
}
