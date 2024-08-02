use serenity::all::{Colour, ComponentInteraction, Context, CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::async_trait;
use crate::component::ComponentTrait;

pub struct NotFountComponent;

#[async_trait]
impl ComponentTrait for NotFountComponent {
    async fn run(ctx: &Context, component: &ComponentInteraction) -> serenity::Result<()> {
        let create_embed = CreateEmbed::new()
            .title("404")
            .description("우와.. 어떻게 이 command를 입력했나요?")
            .color(Colour::new(0xF90707));
        let data = CreateInteractionResponseMessage::new()
            .add_embed(create_embed);
        let builder = CreateInteractionResponse::Message(data);
        if let Err(why) = component.create_response(ctx, builder).await {
            println!("응답할 수 없습니다: {why}")
        }

        Ok(())
    }
}