use serenity::all::{CommandInteraction, CreateEmbed};
use serenity::async_trait;
use crate::commands::{CommandTrait, WOW_DESCRIPTION};
use crate::database::database::{Database, DatabaseTrait};
use crate::global::discord::Discord;
use crate::util::colour::GREEN;

pub struct ResetDBCommand;

#[async_trait]
impl CommandTrait for ResetDBCommand {
    async fn run(discord: &Discord, command: &CommandInteraction) -> serenity::Result<Option<CreateEmbed>> {
        Database.init_entity(discord).await?;
        let create_embed = CreateEmbed::new()
            .title("DB가 초기화 됐습니다.")
            .description(WOW_DESCRIPTION)
            .color(GREEN);
        Ok(Some(create_embed))
    }
}