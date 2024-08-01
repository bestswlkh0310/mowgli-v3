use serenity::all::{CommandInteraction, Context, CreateCommand, CreateEmbed};
use serenity::async_trait;

pub mod get_todos;
pub mod reset_todos;

#[async_trait]
pub trait CommandTrait {
    async fn register() -> CreateCommand;
    async fn run(ctx: &Context, command: &CommandInteraction) -> serenity::Result<CreateEmbed>;
}