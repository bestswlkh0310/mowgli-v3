use serenity::Error;
use crate::database::database::{Database, DatabaseTrait};
use crate::entity::metadata::ChannelId;
use crate::global::discord::Discord;

pub struct MetaDataRepo<'a> {
    discord: &'a Discord<'a>,
}

impl<'a> MetaDataRepo<'a> {
    pub fn new(discord: &'a Discord) -> Self {
        MetaDataRepo { discord }
    }

    pub async fn get_main_channel(&self) -> serenity::Result<ChannelId> {
        let entity = Database.get_entity(self.discord).await?;
        let channel_id = entity.meta_data.main_channel_id.ok_or_else(|| Error::Other("channel_id를 찾을 수 없습니다"))?;
        Ok(channel_id)
    }

    pub async fn edit_main_channel(&self, channel_id: ChannelId) -> serenity::Result<()> {
        let mut entity = Database.get_entity(self.discord).await?;
        entity.meta_data.main_channel_id = Some(channel_id);
        Database.edit_entity(self.discord, &entity).await?;
        Ok(())
    }
}