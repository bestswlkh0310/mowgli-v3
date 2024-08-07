use serenity::{async_trait, Result};
use serenity::all::{ButtonStyle, CommandInteraction, CreateActionRow, CreateButton, CreateInteractionResponse, CreateInteractionResponseMessage, InteractionResponseFlags};

use crate::commands::CommandTrait;
use crate::database::team_repo::TeamRepo;
use crate::global::discord::{Discord, Guild};

pub struct AskTeamCommand;

#[async_trait]
impl CommandTrait for AskTeamCommand {
    async fn run(discord: &Discord, command: &CommandInteraction) -> Result<Option<CreateInteractionResponseMessage>> {
        let teams = TeamRepo::new(Guild::from(discord)).get_teams().await?;
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
            .components(vec![team_selector])
            .flags(InteractionResponseFlags::EPHEMERAL);
        let builder = CreateInteractionResponse::Message(message);
        if let Err(why) = command.create_response(&discord.ctx.http, builder).await {
            println!("응답할 수 없습니다: {why}");
            return Err(why)
        }

        Ok(None)
    }
}