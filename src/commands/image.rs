use hyper::{self, Client};
use hyper::client::response::Response;
use mysql;
use serenity::client::Context;
use serenity::model::{Message, MessageId};

pub const PREFIX: &'static str = "image";

fn load_image(code: &str) -> ::Result<Response> {
    Ok(Client::new()
           .get(&format!("https://i.imgur.com/{}.png", code))
           .send()?)
}

pub fn handle(ctx: Context, msg: &Message, cmd: &str) -> super::CommandResult {
    let db = get_database!(ctx);
    Ok((if let Some(row) = db.prep_exec("SELECT (`code`) FROM images WHERE `name` = ?", (cmd,))?
               .next() {
            let code: String = mysql::from_row(row?);
            let img = load_image(&code)?;
            match img.status {
                hyper::Ok => {
                    Some(msg.channel_id
                             .send_file(img, &[cmd, ".jpg"].concat(), |e| e)?
                             .id)
                }
                _ => None,
            }
        } else {
            None
        },
        None))
}
