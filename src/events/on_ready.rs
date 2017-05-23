use serenity::client::Context;
use serenity::model::Ready;

pub fn handle(ctx: Context, r: Ready) -> ::Result<()> {
    match ctx.data.lock() {
            Ok(data) => data,
            Err(_) => bail!(::ErrorKind::MutexPosioned),
        }
        .insert::<::BotId>(r.user.id);
    Ok(())
}
