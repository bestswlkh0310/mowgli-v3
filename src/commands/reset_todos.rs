use serenity::all::{CommandInteraction, Context, CreateCommand, CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::{async_trait, Error};
use crate::commands::CommandTrait;
use crate::database::database::{Database, DatabaseTrait};
use serenity::Result;

pub struct ResetTodosCommand;

#[async_trait]
impl CommandTrait for ResetTodosCommand {
    async fn register() -> CreateCommand {
        CreateCommand::new("할일초기화")
            .description("할일을 초기화 합니다.")
    }

    async fn run(ctx: &Context, command: &CommandInteraction) -> Result<()> {
        let guild_id = command.guild_id.ok_or_else(|| Error::Other("길드 찾기 실패"))?;
        Database.init_entity(&ctx.http, &guild_id).await?;
        let create_embed = CreateEmbed::new()
            .title("할일 초기화 성공")
            .description("짜잔");

        let data = CreateInteractionResponseMessage::new()
            .add_embed(create_embed);
        let builder = CreateInteractionResponse::Message(data);
        if let Err(why) = command.create_response(&ctx.http, builder).await {
            println!("응답할 수 없습니다: {why}")
        }

        Ok(())
    }
}