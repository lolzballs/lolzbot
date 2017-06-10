use std::io::Cursor;

use serde_json::{self, Value};
use hyper::client::Client;
use hyper::header::Authorization;
use multipart::client::lazy::Multipart;
use mysql;
use serenity::client::Context;
use serenity::model::Message;

pub const PREFIX: &'static str = "upload";

pub fn handle(ctx: Context, msg: &Message, cmd: &str) -> super::CommandResult {
    match ::CONFIG.admins.iter().find(|&&a| a == msg.author.id.0) {
        Some(_) => {
            if msg.attachments.len() == 1 {
                let mut image = match msg.attachments[0].download() {
                    Ok(image) => Cursor::new(image),
                    Err(e) => {
                        return Ok((Some(msg.reply(&format!("Error downloading image: {}", e))?
                                            .id),
                                   None))
                    }
                };
                let res = Multipart::new()
                    .add_stream("image", &mut image, None as Option<&str>, None)
                    .client_request_mut(&Client::new(), "http://api.imgur.com/3/image", |r| {
                        r.header(Authorization(["Client-ID ", &::CONFIG.imgur_id].concat()))
                    })?;
                let res: Value = serde_json::from_reader(res)?;
                match res["success"] {
                    Value::Bool(true) => {
                        match res["data"]["id"] {
                            Value::String(ref code) => {
                                let db = get_database!(ctx);
                                let res = db.prep_exec("INSERT INTO images (`name`, `code`) VALUES (?, ?)",
                                                       (cmd, code));
                                match res {
                                    Ok(res) => {
                                        if res.affected_rows() == 0 {
                                            Ok((None, None))
                                        } else {
                                            Ok((Some(msg.reply("Image uploaded!")?.id), None))
                                        }
                                    }
                                    Err(mysql::Error::MySqlError(mysql::MySqlError {
                                                                     code: 1062, ..
                                                                 })) => {
                                        Ok((Some(msg.reply("Image has duplicate name")?.id), None))
                                    }
                                    Err(e) => Err(e.into()),
                                }
                            }
                            _ => Ok((Some(msg.reply("imgur did something weird!")?.id), None)),
                        }
                    }
                    _ => Ok((Some(msg.reply("imgur failed!")?.id), None)),
                }
            } else {
                Ok((Some(msg.reply("You can only upload one image at a time")?.id), None))
            }
        }
        None => Ok((None, None)),
    }
}
