use serenity::all::{CommandInteraction, CreateInteractionResponseMessage};
use serenity::async_trait;
use crate::commands::CommandTrait;
use crate::global::discord::Discord;

pub struct GetScheduleCommand;

#[async_trait]
impl CommandTrait for GetScheduleCommand {
    async fn run(discord: &Discord, command: &CommandInteraction) -> serenity::Result<Option<CreateInteractionResponseMessage>> {
        Ok(None)
    }
}