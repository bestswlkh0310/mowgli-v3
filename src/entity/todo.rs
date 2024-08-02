use chrono::{Local, NaiveDate};
use serde::{Deserialize, Serialize};
use crate::entity::team::Team;

#[derive(Deserialize, Serialize, Clone)]
pub struct Todo {
    pub team: Team,
    pub todo: TodoContent
}

#[derive(Deserialize, Serialize, Clone)]
pub struct TodoContent {
    pub content: String,
    pub deadline: NaiveDate
}

pub trait VecTodoExtension {
    fn message(&self, team_name: &String) -> String;
}

impl VecTodoExtension for Vec<Todo> {
    fn message(&self, team_name: &String) -> String {
        let mut message = String::new();
        message.push_str(format!("## {}\n", team_name).as_str());
        let mut todos = self.clone();
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
    }
}