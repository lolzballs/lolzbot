use pagination;

use serenity::client::Context;
use serenity::model::Reaction;

pub fn handle(ctx: Context, r: Reaction) -> ::Result<()> {
    pagination::on_reaction(ctx, r)
}
