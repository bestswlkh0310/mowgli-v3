use serenity;
use serenity::{all::GetMessages, async_trait, Error};
use serenity::all::{EditMessage, GuildChannel, Message};
use serenity::Result;

use crate::entity::entity::Entity;
use crate::global::discord::{Guild};
use crate::util::json::to_string;

const DATABASE_CHANNEL: &str = "database-v1";

pub struct Database;

#[async_trait]
pub trait DatabaseTrait: Send + Sync {
    async fn get_entity(&self, guild: &Guild) -> Result<Entity>;
    async fn edit_entity(&self, guild: &Guild, entity: &Entity) -> Result<()>;
    async fn init_entity(&self, guild: &Guild) -> Result<()>;
}

#[async_trait]
impl DatabaseTrait for Database {
    async fn get_entity(&self, guild: &Guild) -> Result<Entity> {
        let message = get_database_message(guild).await?;
        serenity::json::from_str(&message.content)
    }

    async fn edit_entity(&self, guild: &Guild, entity: &Entity) -> Result<()> {
        let mut message = get_database_message(guild).await?;
        let json = to_string(entity)?;
        let builder = EditMessage::new().content(json);
        message.edit(&guild.http, builder).await?;
        Ok(())
    }

    async fn init_entity(&self, guild: &Guild) -> Result<()> {
        let http = &guild.http;
        let channel = get_database_channel(guild).await?;

        // delete all messages
        let builder = GetMessages::new()
            .limit(100);
        let messages = channel.messages(http, builder).await?;
        for message in messages {
            message.delete(http).await?
        }

        // create empty entity
        let entity = Entity::empty();
        let message = to_string(&entity)?;
        channel.say(http, message).await?;
        Ok(())
    }
}

async fn get_database_channel(guild: &Guild) -> Result<GuildChannel> {
    let channels = guild.guild_id.channels(&guild.http).await?;
    let channel = channels.iter()
        .find(|(_, channel)| channel.name == DATABASE_CHANNEL)
        .map(|(_, channel)| channel)
        .ok_or_else(|| Error::Other("채널을 찾을 수 없습니다"))?;
    Ok(channel.clone())
}

async fn get_database_message(guild: &Guild) -> Result<Message> {
    let channel = get_database_channel(guild).await?;
    let builder = GetMessages::new().limit(100);
    let messages = channel.messages(&guild.http, builder).await?;

    let message = messages.get(0)
        .ok_or_else(|| Error::Other("데이터베이스 메세지를 찾을 수 없습니다"))?;

    Ok(message.clone())
}