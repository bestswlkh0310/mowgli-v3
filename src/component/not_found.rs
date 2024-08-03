use serenity::all::{ComponentInteraction, CreateEmbed, CreateInteractionResponseMessage, InteractionResponseFlags};
use serenity::async_trait;
use crate::component::ComponentTrait;
use crate::global::discord::Discord;
use crate::util::colour::RED;

pub struct NotFountComponent;

#[async_trait]
impl ComponentTrait for NotFountComponent {
    async fn run(_discord: &Discord, _component: &ComponentInteraction) -> serenity::Result<Option<CreateInteractionResponseMessage>> {
        let create_embed: CreateEmbed = CreateEmbed::new()
            .title("404")
            .description("우와.. 어떻게 이 명령어를 입력했나요?")
            .color(RED);
        Ok(Some(CreateInteractionResponseMessage::new()
            .flags(InteractionResponseFlags::EPHEMERAL)
            .add_embed(create_embed)))
    }
}