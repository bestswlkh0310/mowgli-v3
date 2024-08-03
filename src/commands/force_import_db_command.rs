use serenity::all::{CommandDataOptionValue, CommandInteraction, CreateEmbed, CreateInteractionResponseMessage, InteractionResponseFlags};
use serenity::{async_trait, Error};
use crate::commands::{CommandTrait, WOW_DESCRIPTION};
use crate::database::database_repo::DatabaseRepo;
use crate::global::discord::{Discord, Guild};
use crate::util::colour::GREEN;

pub struct ForceImportDBCommand;

#[async_trait]
impl CommandTrait for ForceImportDBCommand {
    async fn run(discord: &Discord, command: &CommandInteraction) -> serenity::Result<Option<CreateInteractionResponseMessage>> {
        let option = command.data.options.first().ok_or_else(|| Error::Other("에러"))?;
        println!("{:?}", option);

        if option.name != "force-import" {
            Err(Error::Other("에러"))?
        }

        let command = match &option.value {
            CommandDataOptionValue::SubCommand(value) => value,
            _ => Err(Error::Other("에러"))?
        }.first().ok_or_else(|| Error::Other("에러"))?;

        if command.name != "json" {
            Err(Error::Other("에러"))?
        }

        let json = match &command.value {
            CommandDataOptionValue::String(json) => json,
            _ => Err(Error::Other("에러"))?
        };

        let database_repo = DatabaseRepo::new(Guild::from(discord));
        database_repo.force_import(json).await?;

        let create_embed = CreateEmbed::new()
            .title("DB 강제 불러오기 성공! 🙄")
            .description(WOW_DESCRIPTION)
            .color(GREEN);
        Ok(Some(CreateInteractionResponseMessage::new()
            .flags(InteractionResponseFlags::EPHEMERAL)
            .add_embed(create_embed)))
    }
}