use serenity::{all::{Colour, CommandOptionType, Context, CreateEmbed, ResolvedOption, ResolvedValue}, async_trait, builder::{CreateCommand, CreateCommandOption}, Error, Result};
use serenity::all::{CommandDataOption, CommandInteraction};
use crate::commands::CommandTrait;
use crate::database::database::{Database, DatabaseTrait};
use crate::database::database_mock::DatabaseMock;
use crate::database::team::TeamRepo;
use crate::entity::team::TeamExtension;

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

    async fn run(ctx: &Context, command: &CommandInteraction) -> Result<CreateEmbed> {
        let guild_id = command.guild_id
            .ok_or_else(|| Error::Other("guild id를 찾을 수 없습니다"))?;

        let entity = Database.get_entity(&ctx.http, &guild_id).await?;

        let team_repo = TeamRepo::new(entity);
        let teams = team_repo.get_teams().await?;
        let teams = teams.name_to_lowercase();
        let options = &command.data.options();
        let option = options.get(0)
            .ok_or_else(|| Error::Other("option을 입력해주세요"))?;

        let value = match &option.value {
            ResolvedValue::String(value) if option.name == "팀" && teams.iter().find(|team| team.name.eq(&value.to_lowercase())).is_some() => value.clone(),
            _ => return Err(Error::Other("option값이 잘못되었습니다 Android / iOS / Server / Web 중 하나로 선택해주세요"))
        };

        let entity = team_repo.get_teams().await?;
        let message = serenity::json::to_string(&entity)?;
        let create_embed = CreateEmbed::new()
            .title(format!("{}팀의 할일 입니다", &value.to_lowercase()))
            .description(message)
            .color(Colour::new(0xF4F5F9));
        Ok(create_embed)
    }
}
