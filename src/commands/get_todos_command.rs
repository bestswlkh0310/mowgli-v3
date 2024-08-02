use serenity::{all::{Colour, CreateEmbed, ResolvedValue}, async_trait, Error, Result};
use serenity::all::{CommandDataOption, CommandInteraction, ResolvedOption};

use crate::commands::CommandTrait;
use crate::database::team_repo::TeamRepo;
use crate::database::todo_repo::TodoRepo;
use crate::entity::team::TeamExtension;
use crate::global::discord::Discord;

pub struct GetTodosCommand;

#[async_trait]
impl CommandTrait for GetTodosCommand {
    async fn run(discord: &Discord, command: &CommandInteraction) -> Result<Option<CreateEmbed>> {
        let options = command.data.options();
        let option = options
            .get(0)
            .ok_or_else(|| Error::Other("option을 입력해 주세요"))?;

        let teams = TeamRepo::new(discord)
            .get_teams().await?
            .name_to_lowercase();

        let sub_command = match &option.value {
            ResolvedValue::SubCommand(v) => v,
            _ => return Err(Error::Other("option값이 잘못되었습니다 Android / iOS / Server / Web 중 하나로 선택해주세요"))
        }.first().ok_or_else(|| Error::Other("option값이 잘못되었습니다 Android / iOS / Server / Web 중 하나로 선택해주세요"))?;

        let team_name = match sub_command.value {
            ResolvedValue::String(value) if sub_command.name == "팀" && teams.iter().find(|team| team.name.eq(&value.to_lowercase())).is_some() => value,
            _ => return Err(Error::Other("option값이 잘못되었습니다 Android / iOS / Server / Web 중 하나로 선택해주세요"))
        }.to_lowercase();

        let todo_repo = TodoRepo::new(discord);
        let todos = todo_repo.get_todos_by_team(&team_name).await?;

        let message = {
            let mut todos = todos;
            todos.sort_by(|a, b| a.todo.deadline.cmp(&b.todo.deadline));
            todos.iter().map(|todo| {
                format!("- {} **{}**", todo.todo.deadline.format("%m/%d").to_string(), todo.todo.content)
            }).reduce(|mut a, b| {
                a.push('\n');
                a.push_str(&b);
                a
            })
        }.ok_or_else(|| Error::Other("메세지 파싱 실패"))?;
        let create_embed = CreateEmbed::new()
            .title(format!("{}팀 투두", team_name))
            .description(message)
            .color(Colour::new(0xF4F5F9));

        Ok(Some(create_embed))
    }
}
