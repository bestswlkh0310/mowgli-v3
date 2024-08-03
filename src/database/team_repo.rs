use serenity::Result;

use crate::database::database::{Database, DatabaseTrait};
use crate::entity::team::Team;
use crate::global::discord::{Guild};

pub struct TeamRepo {
    guild: Guild
}

impl TeamRepo {
    pub fn new(guild: Guild) -> Self {
        TeamRepo { guild }
    }

    pub async fn get_teams(&self) -> Result<Vec<Team>> {
        let entity = Database.get_entity(&self.guild).await?;
        let teams = entity.teams.iter().cloned().collect();
        Ok(teams)
    }
}