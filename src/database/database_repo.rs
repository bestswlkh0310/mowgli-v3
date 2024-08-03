use crate::database::database::{Database, DatabaseTrait};
use crate::entity::entity::Entity;
use crate::global::discord::Discord;

pub struct DatabaseRepo<'a> {
    discord: &'a Discord<'a>,
}

impl<'a> DatabaseRepo<'a> {
    pub fn new(discord: &'a Discord) -> Self {
        DatabaseRepo { discord }
    }

    pub async fn force_import(&self, json: &str) -> serenity::Result<()> {
        let entity: Entity = serenity::json::from_str(json)?;
        Database.edit_entity(self.discord, &entity).await
    }
}