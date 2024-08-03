use crate::database::database::{Database, DatabaseTrait};
use crate::entity::entity::Entity;
use crate::global::discord::{Guild};

pub struct DatabaseRepo {
    guild: Guild
}

impl DatabaseRepo {
    pub fn new(guild: Guild) -> Self {
        DatabaseRepo { guild }
    }

    pub async fn force_import(&self, json: &str) -> serenity::Result<()> {
        let entity: Entity = serenity::json::from_str(json)?;
        Database.edit_entity(&self.guild, &entity).await
    }

    pub async fn reset(&self) -> serenity::Result<()> {
        Database.init_entity(&self.guild).await
    }
}