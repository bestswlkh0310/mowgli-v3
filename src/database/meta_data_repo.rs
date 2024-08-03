use serenity::Error;
use crate::database::database::{Database, DatabaseTrait};
use crate::entity::metadata::ChannelId;
use crate::global::discord::{Guild};

pub struct MetaDataRepo {
    guild: Guild,
}

impl MetaDataRepo {
    pub fn new(guild: Guild) -> Self {
        MetaDataRepo { guild }
    }

    pub async fn get_main_channel(&self) -> serenity::Result<ChannelId> {
        let entity = Database.get_entity(&self.guild).await?;
        let channel_id = entity.meta_data.main_channel_id.ok_or_else(|| Error::Other("channel_id를 찾을 수 없습니다"))?;
        Ok(channel_id)
    }

    pub async fn edit_main_channel(&self, channel_id: ChannelId) -> serenity::Result<()> {
        let mut entity = Database.get_entity(&self.guild).await?;
        entity.meta_data.main_channel_id = Some(channel_id);
        Database.edit_entity(&self.guild, &entity).await?;
        Ok(())
    }
}