use serenity::all::CreateEmbed;
use serenity::Result;

pub trait ResultCreateEmbed {
    fn create_embed(&self) -> CreateEmbed;
}

impl ResultCreateEmbed for Result<CreateEmbed> {
    fn create_embed(&self) -> CreateEmbed {
        match self {
            Ok(value) => value.clone(),
            Err(error) => make_create_embed(error.to_string())
        }
    }
}

pub trait UnknownCreateEmbed {
    fn unknown() -> CreateEmbed;
}

impl UnknownCreateEmbed for CreateEmbed {
    fn unknown() -> CreateEmbed {
        make_create_embed("알 수 없는 에러가 발생했습니다".to_string())
    }
}


fn make_create_embed(description: String) -> CreateEmbed {
    CreateEmbed::new()
        .title("Error 😅")
        .description(description)
}