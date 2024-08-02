use serenity::all::{CommandInteraction, CreateEmbed};
use serenity::{async_trait};
use crate::commands::CommandTrait;
use crate::database::database::{Database, DatabaseTrait};
use serenity::Result;
use crate::global::discord::Discord;

pub struct ResetTodosCommand;

#[async_trait]
impl CommandTrait for ResetTodosCommand {
    async fn run(discord: &Discord, _command: &CommandInteraction) -> Result<Option<CreateEmbed>> {
        Database.init_entity(discord).await?;
        let create_embed = CreateEmbed::new()
            .title("투두 초기화 성공")
            .description("짜잔");

        Ok(Some(create_embed))
    }
}