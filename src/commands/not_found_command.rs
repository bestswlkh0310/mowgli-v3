use serenity::all::{Colour, CommandInteraction, CreateEmbed};
use serenity::async_trait;
use crate::commands::CommandTrait;
use crate::global::discord::Discord;

pub struct NotFoundCommand;

#[async_trait]
impl CommandTrait for NotFoundCommand {
    async fn run(_discord: &Discord, _command: &CommandInteraction) -> serenity::Result<Option<CreateEmbed>> {
        let create_embed = CreateEmbed::new()
            .title("404")
            .description("우와.. 어떻게 이 command를 입력했나요?")
            .color(Colour::new(0xF90707));
        Ok(Some(create_embed))
    }
}