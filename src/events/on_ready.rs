use commands;

use serenity::client::Context;
use serenity::model::Ready;

pub fn handle(ctx: Context, r: Ready) {
    *::USER_ID.write().unwrap() = Some(r.user.id);
}
