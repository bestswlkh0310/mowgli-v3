use serenity::all::{Colour, CommandInteraction, Context, CreateCommand, CreateEmbed, CreateInteractionResponseMessage};
use serenity::async_trait;
use serenity::builder::CreateInteractionResponse;
use crate::commands::CommandTrait;

pub struct NotFoundCommand;

#[async_trait]
impl CommandTrait for NotFoundCommand {
    async fn register() -> CreateCommand {
        CreateCommand::new("")
    }

    async fn run(ctx: &Context, command: &CommandInteraction) -> serenity::Result<()> {
        let create_embed = CreateEmbed::new()
            .title("404")
            .description("우와.. 어떻게 이 command를 입력했나요?")
            .color(Colour::new(0xF90707));
        let data = CreateInteractionResponseMessage::new()
            .add_embed(create_embed);
        let builder = CreateInteractionResponse::Message(data);
        if let Err(why) = command.create_response(ctx, builder).await {
            println!("응답할 수 없습니다: {why}")
        }

        Ok(())
    }
}