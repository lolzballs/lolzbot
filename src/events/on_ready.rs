use serenity::client::Context;
use serenity::model::gateway::Ready;

pub fn handle(ctx: Context, r: Ready) -> ::Result<()> {
    ctx.data.write().insert::<::BotId>(r.user.id);
    Ok(())
}
