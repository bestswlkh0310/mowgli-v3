use serenity::all::CreateEmbed;
use serenity::{Error, Result};

pub trait ResultCreateEmbed {
    fn create_embed(&self) -> Option<CreateEmbed>;
}

impl ResultCreateEmbed for Result<Option<CreateEmbed>> {
    fn create_embed(&self) -> Option<CreateEmbed> {
        match self {
            Ok(Some(value)) => Some(value.clone()),
            Ok(None) => None,
            Err(Error::Other(value)) => Some(CreateEmbed::make_create_embed(value.to_string())),
            Err(error) => Some(CreateEmbed::make_create_embed(error.to_string())),
        }
    }
}

pub trait CreateEmbedExtension {
    fn unknown() -> CreateEmbed;
    fn make_create_embed(description: String) -> CreateEmbed;
}

impl CreateEmbedExtension for CreateEmbed {
    fn unknown() -> CreateEmbed {
        Self::make_create_embed("알 수 없는 에러가 발생했습니다".to_string())
    }

    fn make_create_embed(description: String) -> CreateEmbed {
        CreateEmbed::new()
            .title("Error 😅")
            .description(description)
    }
}
