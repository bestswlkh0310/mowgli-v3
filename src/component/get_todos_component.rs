use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{Local, NaiveDate};
use serenity::all::{Colour, ComponentInteraction, CreateEmbed};
use serenity::async_trait;

use crate::component::ComponentTrait;
use crate::database::todo_repo::TodoRepo;
use crate::global::discord::Discord;

pub struct GetTodosComponent;

#[async_trait]
impl ComponentTrait for GetTodosComponent {
    async fn run(discord: &Discord, component: &ComponentInteraction) -> serenity::Result<Option<CreateEmbed>> {
        let team_name = &component.data.custom_id;

        let todo_repo = TodoRepo::new(discord);
        let todos = todo_repo.get_todos_by_team(&team_name).await?;

        let message = {
            let mut message = String::new();
            message.push_str(format!("## {}팀 투두\n", team_name).as_str());
            let mut todos = todos;
            todos.sort_by(|a, b| a.todo.deadline.cmp(&b.todo.deadline));
            todos.iter().for_each(|todo| {
                let now: NaiveDate = Local::now().naive_local().date();
                let is_future = now < todo.todo.deadline;
                let bracket = if is_future { "" } else { "~~" };
                let m = format!(
                    "### - {bracket}{} {}{bracket}\n",
                    todo.todo.deadline.format("%m/%d").to_string(),
                    todo.todo.content,
                );
                message.push_str(&m);
            });
            if todos.is_empty() {
                message.push_str("### 할 일이 없네요. 쉬세요! 🤩")
            };
            message
        };
        let create_embed = CreateEmbed::new()
            .description(message)
            .color(Colour::new(0x33F646));

        Ok(Some(create_embed))
    }
}