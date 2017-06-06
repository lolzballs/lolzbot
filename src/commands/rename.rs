use std::io::Cursor;

use mysql;
use serenity::client::Context;
use serenity::model::{Message, MessageId};

pub const PREFIX: &'static str = "upload";

pub fn handle(ctx: Context, msg: &Message, cmd: &str) -> ::Result<Option<MessageId>> {
    match ::CONFIG.admins.iter().find(|&&a| a == msg.author.id.0) {
        Some(_) => {
            if msg.attachments.len() == 1 {
                let mut image = match msg.attachments[0].download() {
                    Ok(image) => Cursor::new(image),
                    Err(e) => {
                        return Ok(Some(msg.reply(&format!("Error downloading image: {}", e))?
                                           .id))
                    }
                };
                let res = Multipart::new()
                    .add_stream("image", &mut image, None as Option<&str>, None)
                    .client_request_mut(&Client::new(), "https://api.imgur.com/3/image", |r| {
                        r.header(Authorization(["Client-ID ", &::CONFIG.imgur_id].concat()))
                    })?;
                let res: Value = serde_json::from_reader(res)?;
                match res["success"] {
                    Value::Bool(true) => {
                        match res["data"]["id"] {
                            Value::String(ref code) => {
                                let db = {
                                    let data = match ctx.data.lock() {
                                        Ok(data) => data,
                                        Err(_) => bail!(::ErrorKind::MutexPosioned),
                                    };
                                    match data.get::<::DbPool>() {
                                        Some(db) => db.clone(),
                                        None => bail!(::ErrorKind::NoDatabase),
                                    }
                                };
                                db.prep_exec("INSERT INTO images (`name`, `code`) VALUES (?, ?)",
                                               (cmd, code))?;
                                Ok(Some(msg.reply("Image uploaded!")?.id))
                            }
                            _ => Ok(Some(msg.reply("imgur did something weird!")?.id)),
                        }
                    }
                    _ => Ok(Some(msg.reply("imgur failed!")?.id)),
                }
            } else {
                Ok(Some(msg.reply("You can only upload one image at a time")?.id))
            }
        }
        None => Ok(None),
    }
}
