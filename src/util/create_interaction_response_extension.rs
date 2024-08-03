use serenity::all::{CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::builder::CreateAllowedMentions;

pub fn create_response(create_embed: CreateEmbed) -> CreateInteractionResponse {
    let data = CreateInteractionResponseMessage::new()
        .add_embed(create_embed.clone());
    CreateInteractionResponse::Message(data)
}

pub fn create_mentions_all_response() -> CreateInteractionResponse {
    let data = CreateInteractionResponseMessage::new()
        .allowed_mentions(CreateAllowedMentions::new().all_users(true));
    CreateInteractionResponse::Message(data)
}