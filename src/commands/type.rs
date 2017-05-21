use commands;

use serenity::client::Context;
use serenity::model::Ready;

pub fn handle(ctx: Context, ready: Ready) {
    ready.user.id
}
