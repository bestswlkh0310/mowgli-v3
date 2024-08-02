use serenity::all::{Colour, ComponentInteraction, CreateEmbed};
use serenity::async_trait;
use crate::component::ComponentTrait;
use crate::global::discord::Discord;

pub struct NotFountComponent;

#[async_trait]
impl ComponentTrait for NotFountComponent {
    async fn run(_discord: &Discord, _component: &ComponentInteraction) -> serenity::Result<Option<CreateEmbed>> {
        let create_embed = CreateEmbed::new()
            .title("404")
            .description("우와.. 어떻게 이 command를 입력했나요?")
            .color(Colour::new(0xF90707));

        Ok(Some(create_embed))
    }
}