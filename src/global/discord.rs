use std::sync::Arc;
use serenity::all::{Context, GuildId, Http};

pub struct Discord {
    pub ctx: Context,
    pub guild_id: GuildId,
}

impl Discord {
    pub fn new(ctx: Context, guild_id: GuildId) -> Self {
        Discord { ctx, guild_id }
    }
}

pub struct Guild {
    pub http: Arc<Http>,
    pub guild_id: GuildId,
}

impl Guild {
    pub fn new(http: Arc<Http>, guild_id: GuildId) -> Self {
        Guild { http, guild_id }
    }

    pub fn from(discord: &Discord) -> Self {
        Guild::new(discord.ctx.http.clone(), discord.guild_id)
    }
}