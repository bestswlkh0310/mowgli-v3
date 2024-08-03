use serenity::all::{CreateEmbed, CreateInteractionResponseMessage};
use serenity::{Error, Result};

pub trait ResultCreateEmbed {
    fn create_embed(&self) -> Option<CreateInteractionResponseMessage>;
}

impl ResultCreateEmbed for Result<Option<CreateInteractionResponseMessage>> {
    fn create_embed(&self) -> Option<CreateInteractionResponseMessage> {
        match self {
            Ok(Some(value)) => Some(value.clone()),
            Ok(None) => None,
            Err(Error::Other(value)) => Some(
                CreateInteractionResponseMessage::new()
                    .add_embed(CreateEmbed::error_create_embed(value.to_string()))
            ),
            Err(error) => Some(
                CreateInteractionResponseMessage::new()
                    .add_embed(CreateEmbed::error_create_embed(error.to_string()))
            ),
        }
    }
}

pub trait CreateEmbedExtension {
    fn error_create_embed(description: String) -> CreateEmbed;
}

impl CreateEmbedExtension for CreateEmbed {

    fn error_create_embed(description: String) -> CreateEmbed {
        CreateEmbed::new()
            .title("Error 😅")
            .description(description)
    }
}
