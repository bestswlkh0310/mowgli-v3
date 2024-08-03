use serenity::all::{CommandInteraction, CreateEmbed, CreateInteractionResponseMessage};
use serenity::async_trait;
use crate::commands::CommandTrait;
use crate::global::discord::Discord;
use crate::util::colour::RED;

pub struct NotFoundCommand;

#[async_trait]
impl CommandTrait for NotFoundCommand {
    async fn run(_discord: &Discord, _command: &CommandInteraction) -> serenity::Result<Option<CreateInteractionResponseMessage>> {
        let create_embed = CreateEmbed::new()
            .title("404")
            .description("우와.. 어떻게 이 명령어를 입력했나요?")
            .color(RED);
        Ok(Some(CreateInteractionResponseMessage::new()
            .add_embed(create_embed)))
    }
}