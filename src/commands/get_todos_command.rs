use serenity::{all::{Colour, CreateEmbed, ResolvedValue}, async_trait, Error, Result};
use serenity::all::{ButtonStyle, CommandDataOption, CommandInteraction, CreateActionRow, CreateButton, CreateInteractionResponse, CreateInteractionResponseMessage, ResolvedOption};

use crate::commands::CommandTrait;
use crate::database::team_repo::TeamRepo;
use crate::database::todo_repo::TodoRepo;
use crate::entity::team::TeamExtension;
use crate::global::discord::Discord;

pub struct GetTodosCommand;

#[async_trait]
impl CommandTrait for GetTodosCommand {
    async fn run(discord: &Discord, command: &CommandInteraction) -> Result<Option<CreateEmbed>> {
        let teams = TeamRepo::new(discord).get_teams().await?;
        let buttons = teams.iter()
            .map(|team| CreateButton::new(&team.name)
                .label(&team.name)
                .style(ButtonStyle::Secondary)
            )
            .collect();
        let team_selector = CreateActionRow::Buttons(buttons);
        let message = CreateInteractionResponseMessage::new()
            .content(
                "팀을 알려주세요!"
            )
            .components(vec![team_selector]);
        let builder = CreateInteractionResponse::Message(message);
        if let Err(why) = command.create_response(&discord.ctx.http, builder).await {
            println!("응답할 수 없습니다: {why}");
            return Err(why)
        }

        Ok(None)
    }
}
