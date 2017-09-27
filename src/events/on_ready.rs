use serenity::client::Context;
use serenity::model::Ready;

pub fn handle(ctx: Context, r: Ready) -> ::Result<()> {
    ctx.data.lock().insert::<::BotId>(r.user.id);
    Ok(())
}
