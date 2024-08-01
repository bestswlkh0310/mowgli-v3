use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Team {
    pub name: String,
}

pub trait TeamExtension {
    fn name_to_lowercase(&self) -> Vec<Team>;
}

impl TeamExtension for Vec<Team> {
    fn name_to_lowercase(&self) -> Vec<Team> {
        self.into_iter()
            .map(|team| Team {
                name: team.name.to_lowercase(),
            })
            .collect()
    }
}
