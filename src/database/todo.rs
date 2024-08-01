use crate::entity::entity::Entity;
use crate::entity::todo::Todo;
use serenity::Result;

pub struct TodoRepo {
    entity: Entity
}

impl TodoRepo {
    fn new(entity: Entity) -> Self {
        TodoRepo { entity }
    }

    fn get_todos(&self) -> Result<Vec<Todo>> {
        let todos = self.entity.todos.iter().cloned().collect();
        Ok(todos)
    }
}