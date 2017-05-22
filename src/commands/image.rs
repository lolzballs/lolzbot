use std::fs::File;

use serenity::client::Context;
use serenity::model::{Message, MessageId};

pub const PREFIX: &'static str = "image";

fn load_image(path: &str) -> ::Result<File> {
    Ok(File::open(path)?)
}

pub fn handle(_: Context, msg: &Message, cmd: &str) -> ::Result<Option<MessageId>> {
    let filename = [cmd, ".jpg"].concat();
    Ok(match load_image(&filename) {
           Ok(file) => Some(msg.channel_id.send_file(file, &filename, |e| e)?.id),
           Err(_) => None,
       })
}
