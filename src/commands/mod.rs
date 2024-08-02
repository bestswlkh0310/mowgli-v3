use serenity::all::{CommandInteraction, CreateEmbed};
use serenity::async_trait;
use crate::global::discord::Discord;

pub mod get_todos_command;
pub mod reset_todos_command;
pub mod create_todo_command;
pub mod not_found_command;

#[async_trait]
pub trait CommandTrait {
    async fn run(discord: &Discord, command: &CommandInteraction) -> serenity::Result<Option<CreateEmbed>>;
}