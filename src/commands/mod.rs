use serenity::all::{CommandInteraction, CreateEmbed, CreateInteractionResponseMessage};
use serenity::async_trait;
use crate::global::discord::Discord;


pub mod reset_todos_command;
pub mod create_todo_command;
pub mod not_found_command;
pub mod show_all_todos_command;
pub mod remind_command;
pub mod setting_main_channel_command;
pub mod reset_db_command;
pub mod force_import_db_command;

#[async_trait]
pub trait CommandTrait {
    async fn run(discord: &Discord, command: &CommandInteraction) -> serenity::Result<Option<CreateInteractionResponseMessage>>;
}

const WOW_DESCRIPTION: &str = "짜잔";