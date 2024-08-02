use serenity::Result;

use crate::database::database::{Database, DatabaseTrait};
use crate::entity::team::Team;
use crate::global::discord::Discord;

pub struct TeamRepo<'a> {
    discord: &'a Discord<'a>
}

impl<'a> TeamRepo<'a> {
    pub fn new(discord: &'a Discord) -> Self {
        TeamRepo { discord }
    }

    pub async fn get_teams(&self) -> Result<Vec<Team>> {
        let entity = Database.get_entity(self.discord).await?;
        let teams = entity.teams.iter().cloned().collect();
        Ok(teams)
    }
}