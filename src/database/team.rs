use crate::database::database::{Database, DatabaseTrait};
use crate::entity::team::Team;
use serenity::{async_trait, Result};
use crate::entity::entity::Entity;

pub struct TeamRepo<'a> {
    pub entity: &'a Entity,
}

impl<'a> TeamRepo<'a> {
    pub fn new(entity: &'a Entity) -> Self {
        TeamRepo { entity }
    }

    pub fn get_teams(&self) -> Result<Vec<Team>> {
        let teams = self.entity.teams.iter().cloned().collect();
        Ok(teams)
    }
}