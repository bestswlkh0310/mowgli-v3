use crate::entity::entity::Entity;
use crate::entity::todo::Todo;
use serenity::Result;
use crate::database::database::{Database, DatabaseTrait};

pub struct TodoRepo<'a> {
    entity: &'a Entity,
}

impl<'a> TodoRepo<'a> {
    pub fn new(entity: &'a Entity) -> Self {
        TodoRepo { entity }
    }

    pub fn get_todos(&self) -> Result<Vec<Todo>> {
        let todos = self.entity.todos.iter().cloned().collect();
        Ok(todos)
    }

    pub fn get_todos_by_team(&self, team_name: String) -> Result<Vec<Todo>> {
        let todos = self.get_todos()?;
        let todos = todos.iter().filter(|todo| todo.team.name == team_name.to_lowercase()).cloned().collect();
        Ok(todos)
    }

    pub fn create_todo(&self, todo: Todo) -> Result<()> {
        let mut todos = self.get_todos()?;
        todos.push(todo);
        Ok(())
    }
}