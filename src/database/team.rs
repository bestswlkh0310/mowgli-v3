use crate::database::database::{Database, DatabaseTrait};
use crate::entity::team::Team;
use serenity::{async_trait, Result};
use crate::entity::entity::Entity;

pub struct TeamRepo {
    pub entity: Entity,
}

impl TeamRepo {

    pub fn new(entity: Entity) -> Self {
        TeamRepo { entity }
    }

    pub async fn get_teams(&self) -> Result<Vec<Team>> {
        let teams = self.entity.teams.iter().cloned().collect();
        Ok(teams)
    }
}