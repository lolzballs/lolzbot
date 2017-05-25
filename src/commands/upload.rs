use std::fs::OpenOptions;
use std::io::Write;

use serenity::client::Context;
use serenity::model::{Message, MessageId};

pub const PREFIX: &'static str = "upload";

pub fn handle(_: Context, msg: &Message, cmd: &str) -> ::Result<Option<MessageId>> {
    if ::CONFIG.owner == msg.author.id.0 {
        if msg.attachments.len() == 1 {
            let mut file = match OpenOptions::new()
                      .write(true)
                      .create_new(true)
                      .open([cmd, ".jpg"].concat()) {
                Ok(file) => file,
                Err(e) => return Ok(Some(msg.reply(&format!("Error creating file: {}", e))?.id)),
            };
            let image = match msg.attachments[0].download() {
                Ok(image) => image,
                Err(e) => {
                    return Ok(Some(msg.reply(&format!("Error downloading image: {}", e))?
                                       .id))
                }
            };
            match file.write_all(&image) {
                Ok(_) => Ok(Some(msg.reply("Image created!")?.id)),
                Err(e) => Ok(Some(msg.reply(&format!("Error writing to file: {}", e))?.id)),
            }
        } else {
            Ok(Some(msg.reply("You can only upload one image at a time")?.id))
        }
    } else {
        Ok(None)
    }
}
