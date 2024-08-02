use serenity::all::{ComponentInteraction, CreateEmbed};
use serenity::async_trait;

use crate::component::ComponentTrait;
use crate::database::todo_repo::TodoRepo;
use crate::entity::todo::VecTodoExtension;
use crate::global::discord::Discord;
use crate::util::colour::GREEN;

pub struct ShowTodosComponent;

#[async_trait]
impl ComponentTrait for ShowTodosComponent {
    async fn run(discord: &Discord, component: &ComponentInteraction) -> serenity::Result<Option<CreateEmbed>> {
        let team_name = &component.data.custom_id;

        let todo_repo = TodoRepo::new(discord);
        let todos = todo_repo.get_todos_by_team(&team_name).await?;

        let message = todos.message(team_name);
        let create_embed = CreateEmbed::new()
            .description(message)
            .color(GREEN);

        Ok(Some(create_embed))
    }
}