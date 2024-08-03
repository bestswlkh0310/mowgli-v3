use serenity::all::{ComponentInteraction, CreateInteractionResponseMessage};
use serenity::async_trait;
use serenity::Result;
use crate::global::discord::Discord;

pub mod create_todo_component;
pub mod not_found;
pub mod show_todos_component;

#[async_trait]
pub trait ComponentTrait {
    async fn run(discord: &Discord, component: &ComponentInteraction) -> Result<Option<CreateInteractionResponseMessage>>;
}