use serde::{Deserialize, Serialize};
use crate::entity::metadata::MetaData;
use crate::entity::team::Team;
use crate::entity::todo::Todo;

#[derive(Deserialize, Serialize, Clone)]
pub struct Entity {
    pub teams: Vec<Team>,
    pub todos: Vec<Todo>,
    pub meta_data: MetaData
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
            meta_data: MetaData {
                main_channel_id: None
            }
        }
    }
}