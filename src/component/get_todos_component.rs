use serenity::all::{Colour, ComponentInteraction, CreateEmbed, ResolvedValue};
use serenity::{async_trait, Error};
use crate::component::ComponentTrait;
use crate::database::todo_repo::TodoRepo;
use crate::global::discord::Discord;

pub struct GetTodosComponent;

#[async_trait]
impl ComponentTrait for GetTodosComponent {
    async fn run(discord: &Discord, component: &ComponentInteraction) -> serenity::Result<Option<CreateEmbed>> {
        let team_name = &component.data.custom_id.to_lowercase();

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
        }.unwrap_or("할 일이 없네요. 쉬세요! 🤩".to_string());
        let create_embed = CreateEmbed::new()
            .title(format!("{}팀 투두", team_name))
            .description(message)
            .color(Colour::new(0xF4F5F9));

        Ok(Some(create_embed))
    }
}