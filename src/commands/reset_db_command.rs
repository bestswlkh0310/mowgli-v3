use serenity::all::{CommandInteraction, CreateEmbed, CreateInteractionResponseMessage, InteractionResponseFlags};
use serenity::async_trait;
use crate::commands::{CommandTrait, WOW_DESCRIPTION};
use crate::database::database_repo::DatabaseRepo;
use crate::global::discord::{Discord, Guild};
use crate::util::colour::GREEN;

pub struct ResetDBCommand;

#[async_trait]
impl CommandTrait for ResetDBCommand {
    async fn run(discord: &Discord, _command: &CommandInteraction) -> serenity::Result<Option<CreateInteractionResponseMessage>> {
        DatabaseRepo::new(Guild::from(discord)).reset().await?;
        let create_embed = CreateEmbed::new()
            .title("DB가 초기화 됐습니다.")
            .description(WOW_DESCRIPTION)
            .color(GREEN);
        Ok(Some(CreateInteractionResponseMessage::new()
            .flags(InteractionResponseFlags::EPHEMERAL)
            .add_embed(create_embed)))
    }
}