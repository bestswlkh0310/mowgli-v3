use serenity::{all::{Colour, CommandOptionType, CreateEmbed, ResolvedValue}, async_trait, builder::{CreateCommand, CreateCommandOption}, Error, Result};

use crate::commands::CommandTrait;
use crate::database::team_repo::TeamRepo;
use crate::database::todo_repo::TodoRepo;
use crate::entity::team::TeamExtension;
use crate::global::discord::Discord;
use crate::util::json::to_string;

use serenity::all::{CommandInteraction};
pub struct GetTodosCommand;

#[async_trait]
impl CommandTrait for GetTodosCommand {
    async fn register() -> CreateCommand {
        CreateCommand::new("할일")
            .description("할일을 확인합니다")
            .add_option(
                CreateCommandOption::new(CommandOptionType::String, "팀", "Android / iOS / Server / Web")
                    .required(true),
            )
    }

    async fn run(discord: &Discord, command: &CommandInteraction) -> Result<Option<CreateEmbed>> {
        let options = command.data.options();
        let option = options
            .get(0)
            .ok_or_else(|| Error::Other("option을 입력해 주세요"))?;

        let teams = TeamRepo::new(discord)
            .get_teams().await?
            .name_to_lowercase();

        let team_name = match &option.value {
            ResolvedValue::String(value) if option.name == "팀" && teams.iter().find(|team| team.name.eq(&value.to_lowercase())).is_some() => value,
            _ => return Err(Error::Other("option값이 잘못되었습니다 Android / iOS / Server / Web 중 하나로 선택해주세요"))
        }.to_lowercase();

        let todo_repo = TodoRepo::new(discord);
        let todos = todo_repo.get_todos_by_team(&team_name).await?;

        let message = to_string(&todos)?;
        let create_embed = CreateEmbed::new()
            .title(format!("{}팀의 할일 입니다", team_name))
            .description(message)
            .color(Colour::new(0xF4F5F9));

        Ok(Some(create_embed))
    }
}
