use serenity::all::{CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage};

pub fn create_response(create_embed: CreateEmbed) -> CreateInteractionResponse {
    let data = CreateInteractionResponseMessage::new()
        .add_embed(create_embed.clone());
    CreateInteractionResponse::Message(data)
}