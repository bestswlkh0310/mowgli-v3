use serenity::all::{ComponentInteraction, Context, CreateCommand};
use serenity::async_trait;

pub mod create_todo;
pub mod not_found;

#[async_trait]
pub trait ComponentTrait {
    async fn run(ctx: &Context, component: &ComponentInteraction) -> serenity::Result<()>;
}