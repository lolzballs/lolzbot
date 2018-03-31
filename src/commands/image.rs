use mysql;
use serenity::client::Context;
use serenity::model::Message;
use std::path::PathBuf;

pub const PREFIX: &'static str = "image";

pub fn handle(ctx: Context, msg: &Message, cmd: &str) -> super::CommandResult {
    let db = get_data!(ctx, ::DbPool);
    Ok((
        if let Some(row) = db.prep_exec(
            "SELECT (`code`) FROM images WHERE `name` = ?",
            (cmd,),
        )?
            .next()
        {
            let code: String = mysql::from_row(row?);
            let mut file_path: PathBuf = [&::CONFIG.image_path, &code].iter().collect();
            file_path.set_extension("png");
            Some(msg.channel_id.send_files(vec![&file_path], |m| m)?.id)
        } else {
            None
        },
        None,
    ))
}
