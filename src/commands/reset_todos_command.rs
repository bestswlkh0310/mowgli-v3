use serenity::all::{CommandInteraction, CreateEmbed, CreateInteractionResponseMessage, InteractionResponseFlags};
use serenity::{async_trait};
use crate::commands::{CommandTrait, WOW_DESCRIPTION};
use serenity::Result;
use crate::database::todo_repo::TodoRepo;
use crate::global::discord::{Discord, Guild};

pub struct ResetTodosCommand;

#[async_trait]
impl CommandTrait for ResetTodosCommand {
    async fn run(discord: &Discord, _command: &CommandInteraction) -> Result<Option<CreateInteractionResponseMessage>> {
        let todo_repo = TodoRepo::new(Guild::from(discord));
        todo_repo.reset_todo().await?;

        let create_embed = CreateEmbed::new()
            .title("투두 초기화 성공")
            .description(WOW_DESCRIPTION);

        Ok(Some(CreateInteractionResponseMessage::new()
            .flags(InteractionResponseFlags::EPHEMERAL)
            .add_embed(create_embed)))
    }
}