use serde::{Deserialize, Serialize};
use crate::entity::team::Team;
use crate::entity::todo::Todo;

#[derive(Deserialize, Serialize)]
pub struct Entity {
    pub teams: Vec<Team>,
    pub todos: Vec<Todo>,
}

impl Entity {
    pub fn empty() -> Entity {
        Entity {
            teams: vec![
                Team {
                    name: String::from("iOS"),
                },
                Team {
                    name: String::from("Android")
                },
                Team {
                    name: String::from("Web")
                },
                Team {
                    name: String::from("Server")
                },
            ],
            todos: vec![],
        }
    }
}