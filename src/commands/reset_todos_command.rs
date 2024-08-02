use serenity::all::{CommandInteraction, CreateCommand, CreateEmbed};
use serenity::{async_trait};
use crate::commands::CommandTrait;
use crate::database::database::{Database, DatabaseTrait};
use serenity::Result;
use crate::global::discord::Discord;

pub struct ResetTodosCommand;

#[async_trait]
impl CommandTrait for ResetTodosCommand {
    async fn register() -> CreateCommand {
        CreateCommand::new("할일초기화")
            .description("할일을 초기화 합니다.")
    }

    async fn run(discord: &Discord, _command: &CommandInteraction) -> Result<Option<CreateEmbed>> {
        Database.init_entity(discord).await?;
        let create_embed = CreateEmbed::new()
            .title("할일 초기화 성공")
            .description("짜잔");

        Ok(Some(create_embed))
    }
}