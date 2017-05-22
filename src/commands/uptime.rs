use std::time::Instant;

use serenity::client::Context;
use serenity::model::{Message, MessageId};

pub const PREFIX: &'static str = "uptime";

pub fn handle(ctx: Context, msg: &Message, _: &str) -> ::Result<Option<MessageId>> {
    let duration = {
        let data = ctx.data.lock().unwrap();
        let start = match data.get::<::StartTime>() {
            Some(time) => time,
            None => bail!("No start time!"),
        };
        Instant::now().duration_since(*start)
    };
    let delta = duration.as_secs();
    let (hours, remainder) = (delta / 3600, delta % 3600);
    let (minutes, seconds) = (remainder / 60, remainder % 60);
    let (days, hours) = (hours / 24, hours % 24);

    let days_str = if days != 0 {
        Some(days.to_string())
    } else {
        None
    };
    let days_txt = if days == 1 {
        " day, "
    } else if days != 0 {
        " days, "
    } else {
        ""
    };
    let hours_str = if hours != 0 {
        Some(hours.to_string())
    } else {
        None
    };
    let hours_txt = if hours == 1 {
        " hour, "
    } else if hours != 0 {
        " hours, "
    } else {
        ""
    };
    let minutes_str = if minutes != 0 {
        Some(minutes.to_string())
    } else {
        None
    };
    let minutes_txt = if minutes == 1 {
        " minute, and "
    } else if minutes != 0 {
        " minutes, and "
    } else {
        ""
    };
    let seconds_txt = if seconds == 1 {
        " second!"
    } else {
        " seconds!"
    };

    let message = ["This bot has been up for ",
                   days_str.as_ref().map_or("", |s| s.as_str()),
                   days_txt,
                   hours_str.as_ref().map_or("", |s| s.as_str()),
                   hours_txt,
                   minutes_str.as_ref().map_or("", |s| s.as_str()),
                   minutes_txt,
                   seconds.to_string().as_str(),
                   seconds_txt]
            .concat();
    Ok(Some(msg.reply(&message)?.id))
}
