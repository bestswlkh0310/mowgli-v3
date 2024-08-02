use crate::entity::todo::Todo;
use serenity::Result;
use crate::database::database::{Database, DatabaseTrait};
use crate::global::discord::Discord;

pub struct TodoRepo<'a> {
    pub discord: &'a Discord<'a>,
}

impl<'a> TodoRepo<'a> {
    pub fn new(discord: &'a Discord) -> Self {
        TodoRepo { discord }
    }

    pub async fn get_todos(&self) -> Result<Vec<Todo>> {
        let entity = Database.get_entity(self.discord).await?;
        let todos = entity.todos.iter().cloned().collect();
        Ok(todos)
    }

    pub async fn get_todos_by_team(&self, team_name: &String) -> Result<Vec<Todo>> {
        let todos = self.get_todos().await?;
        let todos = todos.iter().filter(|todo| todo.team.name == team_name.clone()).cloned().collect();
        Ok(todos)
    }

    pub async fn create_todo(&self, todo: &Todo) -> Result<()> {
        let mut entity = Database.get_entity(self.discord).await?;
        entity.todos.push(todo.clone());
        Database.edit_entity(self.discord, &entity).await?;
        Ok(())
    }
}