use std::fs::File;

use serenity::client::Context;
use serenity::model::Message;

pub const PREFIX: &'static str = "image";

fn load_image(path: &str) -> ::Result<File> {
    Ok(File::open(path)?)
}

pub fn handle(ctx: Context, msg: &Message, cmd: &str) {
    let filename = [cmd, ".jpg"].join("");
    match load_image(&filename) {
        Ok(file) => {
            msg.channel_id
                .send_file(file, &filename, |e| e)
                .unwrap();
        }
        Err(e) => println!("{}", e),
    };
}
