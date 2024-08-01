use chrono::NaiveDate;
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