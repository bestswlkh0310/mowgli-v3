use serenity::all::{CommandInteraction, CreateInteractionResponseMessage};
use serenity::async_trait;
use crate::commands::CommandTrait;
use crate::global::discord::Discord;

pub struct CreateScheduleCommand;

#[async_trait]
impl CommandTrait for CreateScheduleCommand {
    async fn run(discord: &Discord, command: &CommandInteraction) -> serenity::Result<Option<CreateInteractionResponseMessage>> {
        Ok(None)
    }
}