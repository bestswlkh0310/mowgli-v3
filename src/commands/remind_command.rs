use serenity::all::{CommandInteraction, CreateEmbed};
use serenity::async_trait;
use crate::commands::CommandTrait;
use crate::global::discord::Discord;
use crate::util::create_interaction_response_extension::create_mentions_all_response;

pub struct RemindCommand;

#[async_trait]
impl CommandTrait for RemindCommand {
    async fn run(discord: &Discord, command: &CommandInteraction) -> serenity::Result<Option<CreateEmbed>> {
        let builder = create_mentions_all_response();
        if let Err(why) = command.create_response(&discord.ctx.http, builder).await {
            println!("{} Err - {}", file!(), why);
        }
        Ok(None)
    }
}