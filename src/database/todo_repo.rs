use crate::entity::todo::Todo;
use serenity::Result;
use crate::database::database::{Database, DatabaseTrait};
use crate::global::discord::{Guild};

pub struct TodoRepo {
    pub guild: Guild
}

impl TodoRepo {
    pub fn new(guild: Guild) -> Self {
        TodoRepo { guild }
    }

    pub async fn get_todos(&self) -> Result<Vec<Todo>> {
        let entity = Database.get_entity(&self.guild).await?;
        let todos = entity.todos.iter().cloned().collect();
        Ok(todos)
    }

    pub async fn get_todos_by_team(&self, team_name: &String) -> Result<Vec<Todo>> {
        let todos = self.get_todos().await?;
        let todos = todos.iter().filter(|todo| todo.team.name == team_name.clone()).cloned().collect();
        Ok(todos)
    }

    pub async fn create_todo(&self, todo: &Todo) -> Result<()> {
        let mut entity = Database.get_entity(&self.guild).await?;
        entity.todos.push(todo.clone());
        Database.edit_entity(&self.guild, &entity).await?;
        Ok(())
    }

    pub async fn reset_todo(&self) -> Result<()> {
        let mut entity = Database.get_entity(&self.guild).await?;
        entity.todos = vec![];
        Database.edit_entity(&self.guild, &entity).await?;
        Ok(())
    }
}