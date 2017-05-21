use serde_json;
use serenity;
use std;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        SerdeJson(serde_json::Error);
        Serenity(serenity::Error);
    }
}
