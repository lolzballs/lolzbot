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

pub fn handle(ctx: Context, msg: &Message, cmd: &str) -> ::Result<Option<MessageId>> {
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

    Ok(if let Some(row) = db.prep_exec("SELECT (`code`) FROM images WHERE `name` = ?", (cmd,))?
              .next() {
           let code: String = mysql::from_row(row?);
           let img = load_image(&code)?;
           match img.status {
               hyper::Ok => {
                   Some(msg.channel_id
                            .send_file(img, &[&code, ".jpg"].concat(), |e| e)?
                            .id)
               }
               _ => None,
           }
       } else {
           None
       })
}
