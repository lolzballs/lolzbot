use serenity::client::Context;
use serenity::model::Ready;

pub fn handle(ctx: Context, r: Ready) {
    ctx.data.lock().unwrap().insert::<::BotId>(r.user.id);
}
