use std::time::Instant;

use serenity::client::Context;
use serenity::model::{Message, MessageId};

pub const PREFIX: &'static str = "uptime";

pub fn handle(ctx: Context, msg: &Message, _: &str) -> Option<MessageId> {
    let duration =
        Instant::now().duration_since(*ctx.data.lock().unwrap().get::<::StartTime>().unwrap());
    let delta = duration.as_secs();
    let (hours, remainder) = (delta / 3600, delta % 3600);
    let (minutes, seconds) = (remainder / 60, remainder % 60);
    let (days, hours) = (hours / 24, hours % 24);

    let mut message = String::from("This bot has been up for ");
    if days != 0 {
        message.push_str(days.to_string().as_str());
        message.push_str(if days == 1 { " day, " } else { " days, " });
    }
    if hours != 0 {
        message.push_str(hours.to_string().as_str());
        message.push_str(if hours == 1 { " hour, " } else { " hours, " });
    }
    if minutes != 0 {
        message.push_str(minutes.to_string().as_str());
        message.push_str(if minutes == 1 {
                             " minute,  and "
                         } else {
                             " minutes, and "
                         });
    }

    message.push_str(seconds.to_string().as_str());
    message.push_str(if seconds == 1 {
                         " second."
                     } else {
                         " seconds."
                     });
    Some(msg.reply(&message).unwrap().id)
}
