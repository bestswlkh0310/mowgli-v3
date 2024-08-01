use serenity::all::{GuildId, Http};
use crate::database::database::DatabaseTrait;
use crate::entity::entity::Entity;
use serenity::{async_trait, Result};

pub struct DatabaseMock;

#[async_trait]
impl DatabaseTrait for DatabaseMock {
    async fn get_entity(&self, http: &Http, guild_id: &GuildId) -> Result<Entity> {
        let entity = Entity::empty();
        Ok(entity)
    }

    async fn edit_entity(&self, http: &Http, guild_id: &GuildId, content: &Entity) -> Result<()> {
        Ok(())
    }

    async fn init_entity(&self, http: &Http, guild_id: &GuildId) -> Result<()> {
        Ok(())
    }
}