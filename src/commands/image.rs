use std::fs::File;

use serenity::client::Context;
use serenity::model::{Message, MessageId};

pub const PREFIX: &'static str = "image";

fn load_image(path: &str) -> ::Result<File> {
    Ok(File::open(path)?)
}

pub fn handle(_: Context, msg: &Message, cmd: &str) -> Option<MessageId> {
    let filename = [cmd, ".jpg"].join("");
    match load_image(&filename) {
        Ok(file) => {
            Some(msg.channel_id
                     .send_file(file, &filename, |e| e)
                     .unwrap()
                     .id)
        }
        Err(e) => None,
    }
}
