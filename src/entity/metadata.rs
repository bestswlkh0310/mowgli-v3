use serde::{Deserialize, Serialize};

pub type ChannelId = u64;

#[derive(Deserialize, Serialize, Clone)]
pub struct MetaData {
    pub main_channel_id: Option<ChannelId>
}