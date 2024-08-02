use std::any::Any;
use chrono::{NaiveDate, Utc};
use serenity::{async_trait, Error, Result};
use serenity::all::{ButtonStyle, CommandInteraction, ComponentType, Context, CreateActionRow, CreateButton, CreateCommand, CreateEmbed, CreateInputText, CreateInteractionResponse, CreateInteractionResponseMessage, CreateModal, CreateQuickModal, InputText, InputTextStyle};
use crate::commands::CommandTrait;
use crate::database::database::{Database, DatabaseTrait};
use crate::database::team::TeamRepo;
use crate::entity::team::Team;
use crate::entity::todo::{Todo, TodoContent};

pub struct CreateTodoCommand;

#[async_trait]
impl CommandTrait for CreateTodoCommand {
    async fn register() -> CreateCommand {
        CreateCommand::new("할일추가")
            .description("할일을 추가합니다.")
    }

    async fn run(ctx: &Context, command: &CommandInteraction) -> Result<()> {
        let guild_id = command.guild_id
            .ok_or_else(|| Error::Other("길드를 찾을 수 없습니다"))?;
        let mut entity = Database.get_entity(&ctx.http, &guild_id).await?;

        let teams = TeamRepo::new(&entity).get_teams()?;

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
        if let Err(why) = command.create_response(&ctx.http, builder).await {
            println!("응답할 수 없습니다: {why}");
            return Err(why)
        }

        return Ok(());
        // let (team, content, deadline) = (&inputs[0], &inputs[1], &inputs[2]);

        let new_todo = Todo {
            team: Team {
                name: String::from("iOS").to_lowercase()
            },
            todo: TodoContent {
                content: String::from("WOW"),
                deadline: Utc::now().naive_utc().date(),
            },
        };
        entity.todos.push(new_todo);
        Database.edit_entity(&ctx.http, &guild_id, &entity).await?;

        let create_embed = CreateEmbed::new()
            .title("할일 추가 완료")
            .description("야호");

        let data = CreateInteractionResponseMessage::new()
            .add_embed(create_embed);
        let builder = CreateInteractionResponse::Message(data);
        // if let Err(why) = response.interaction.create_response(&ctx.http, builder).await {
        //     println!("응답할 수 없습니다: {why}")
        // }

        Ok(())
    }
}