use dotenv::dotenv;
use serenity;
use serenity::{all::GetMessages, Error, all::GuildId, prelude::Context, async_trait};
use serenity::all::{EditMessage, GuildChannel, Http, Message};
use serenity::Result;
use crate::entity::entity::Entity;

const DATABASE_CHANNEL: &str = "database-v1";

pub struct Database;

#[async_trait]
pub trait DatabaseTrait: Send + Sync {
    async fn get_entity(&self, http: &Http, guild_id: &GuildId) -> Result<Entity>;
    async fn edit_entity(&self, http: &Http, guild_id: &GuildId, entity: &Entity) -> Result<()>;
    async fn init_entity(&self, http: &Http, guild_id: &GuildId) -> Result<()>;
}

#[async_trait]
impl DatabaseTrait for Database {

    async fn get_entity(&self, http: &Http, guild_id: &GuildId) -> Result<Entity> {
        let message = get_database_message(http, guild_id).await?;
        serenity::json::from_str(&message.content)
    }

    async fn edit_entity(&self, http: &Http, guild_id: &GuildId, entity: &Entity) -> Result<()>{
        let mut message = get_database_message(&http, guild_id).await?;
        let json = serenity::json::to_string(entity)?;
        let builder = EditMessage::new().content(json);
        message.edit(&http, builder).await?;
        Ok(())
    }

    async fn init_entity(&self, http: &Http, guild_id: &GuildId) -> Result<()> {
        let channel = get_database_channel(http, guild_id).await?;
        let entity = Entity::empty();
        let message = serenity::json::to_string(&entity)?;
        channel.say(http, message).await?;
        Ok(())
    }
}

async fn get_database_channel(http: &Http, guild_id: &GuildId) -> Result<GuildChannel> {
    let channels = guild_id.channels(&http).await?;
    let channel = channels.iter()
        .find(|(_, channel)| channel.name == DATABASE_CHANNEL)
        .map(|(_, channel)| channel)
        .ok_or_else(|| Error::Other("채널을 찾을 수 없습니다"))?;
    Ok(channel.clone())
}

async fn get_database_message(http: &Http, guild_id: &GuildId) -> Result<Message> {
    let channel = get_database_channel(http, guild_id).await?;
    let builder = GetMessages::new().limit(100);
    let messages = channel.messages(&http, builder).await?;

    let message = messages.get(0)
        .ok_or_else(|| Error::Other("데이터베이스 메세지를 찾을 수 없습니다"))?;

    Ok(message.clone())
}