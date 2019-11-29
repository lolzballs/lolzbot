use pagination;

use serenity::client::Context;
use serenity::model::channel::Reaction;

pub fn handle(ctx: Context, r: Reaction) -> ::Result<()> {
    pagination::on_reaction(ctx, r)
}
