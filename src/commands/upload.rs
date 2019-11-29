use std::fs::File;
use std::io::Write;
use std::iter;
use std::path::PathBuf;

use mysql;
use rand::distributions::Alphanumeric;
use rand::{self, Rng};
use serenity::client::Context;
use serenity::model::channel::Message;

pub const PREFIX: &'static str = "upload";

fn generate_code() -> String {
    let mut s;
    while {
        let mut rng = rand::thread_rng();
        s = iter::repeat(())
            .map(|()| rng.sample(Alphanumeric))
            .take(7)
            .collect();
        let mut file_path: PathBuf = [&::CONFIG.image_path, &s].iter().collect();
        file_path.set_extension("png");
        file_path.exists()
    } {}
    s
}

pub fn handle(ctx: &Context, msg: &Message, cmd: &str) -> super::CommandResult {
    match ::CONFIG.admins.iter().find(|&&a| a == msg.author.id.0) {
        Some(_) => {
            if msg.attachments.len() == 1 {
                let image = match msg.attachments[0].download() {
                    Ok(image) => image,
                    Err(e) => {
                        return Ok((
                            Some(
                                msg.reply(
                                    ctx,
                                    &format!("Error downloading image from Discord: {}", e),
                                )?
                                .id,
                            ),
                            None,
                        ))
                    }
                };
                let db = get_data!(ctx, ::DbPool);
                let code = generate_code();
                let mut file = {
                    let mut file_path: PathBuf = [&::CONFIG.image_path, &code].iter().collect();
                    file_path.set_extension("png");
                    File::create(file_path)
                }?;
                file.write_all(&image)?;
                let res = db.prep_exec(
                    "INSERT INTO images (`name`, `code`) VALUES (?, ?)",
                    (cmd, code),
                );
                match res {
                    Ok(res) => {
                        if res.affected_rows() == 0 {
                            Ok((None, None))
                        } else {
                            Ok((Some(msg.reply(ctx, "Image uploaded!")?.id), None))
                        }
                    }
                    Err(mysql::Error::MySqlError(mysql::MySqlError { code: 1062, .. })) => {
                        Ok((Some(msg.reply(ctx, "Image has duplicate name")?.id), None))
                    }
                    Err(e) => Err(e.into()),
                }
            } else {
                Ok((
                    Some(
                        msg.reply(ctx, "You can only upload one image at a time")?
                            .id,
                    ),
                    None,
                ))
            }
        }
        None => Ok((None, None)),
    }
}
