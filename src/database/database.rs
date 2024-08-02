use serenity;
use serenity::{all::GetMessages, async_trait, Error};
use serenity::all::{EditMessage, GuildChannel, Message};
use serenity::Result;

use crate::entity::entity::Entity;
use crate::global::discord::Discord;
use crate::util::json::to_string;

const DATABASE_CHANNEL: &str = "database-v1";

pub struct Database;

#[async_trait]
pub trait DatabaseTrait<'a>: Send + Sync {
    async fn get_entity(&self, discord: &'a Discord) -> Result<Entity>;
    async fn edit_entity(&self, discord: &'a Discord, entity: &Entity) -> Result<()>;
    async fn init_entity(&self, discord: &'a Discord) -> Result<()>;
}

#[async_trait]
impl<'a> DatabaseTrait<'a> for Database {
    async fn get_entity(&self, discord: &'a Discord) -> Result<Entity> {
        let message = get_database_message(discord).await?;
        serenity::json::from_str(&message.content)
    }

    async fn edit_entity(&self, discord: &'a Discord, entity: &Entity) -> Result<()> {
        let mut message = get_database_message(discord).await?;
        let json = to_string(entity)?;
        let builder = EditMessage::new().content(json);
        message.edit(&discord.ctx.http, builder).await?;
        Ok(())
    }

    async fn init_entity(&self, discord: &'a Discord) -> Result<()> {
        let http = &discord.ctx.http;
        let channel = get_database_channel(discord).await?;

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

async fn get_database_channel(discord: &Discord<'_>) -> Result<GuildChannel> {
    let channels = discord.guild_id.channels(&discord.ctx.http).await?;
    let channel = channels.iter()
        .find(|(_, channel)| channel.name == DATABASE_CHANNEL)
        .map(|(_, channel)| channel)
        .ok_or_else(|| Error::Other("채널을 찾을 수 없습니다"))?;
    Ok(channel.clone())
}

async fn get_database_message(discord: &Discord<'_>) -> Result<Message> {
    let channel = get_database_channel(discord).await?;
    let builder = GetMessages::new().limit(100);
    let messages = channel.messages(&discord.ctx.http, builder).await?;

    let message = messages.get(0)
        .ok_or_else(|| Error::Other("데이터베이스 메세지를 찾을 수 없습니다"))?;

    Ok(message.clone())
}