use std::time::SystemTime;

use serenity::client::Context;
use serenity::model::Message;

pub const PREFIX: &'static str = "uptime";

pub fn handle(ctx: Context, msg: &Message, _: &str) {
    let duration = SystemTime::now()
        .duration_since(*ctx.data.lock().unwrap().get::<::StartTime>().unwrap())
        .unwrap();
    msg.reply(&format!("This bot has been up for {} second(s)!", duration.as_secs()))
        .unwrap();
}
