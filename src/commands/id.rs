use mysql;
use serenity::client::Context;
use serenity::model::Message;

pub const PREFIX: &'static str = "id";

pub fn handle(ctx: Context, msg: &Message, cmd: &str) -> super::CommandResult {
    let db = get_data!(ctx, ::DbPool);
    if let Some(row) = db.prep_exec("SELECT (`id`) FROM images WHERE `name` = ?", (cmd,))?
        .next()
    {
        let id: u64 = mysql::from_row(row?);
        Ok((
            Some(
                msg.reply(&format!("Image `{}` has id `{}`", cmd, id))?.id,
            ),
            None,
        ))
    } else {
        Ok((None, None))
    }
}
