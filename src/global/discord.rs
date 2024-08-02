use serenity::all::{Context, GuildId};

pub struct Discord<'a> {
    pub ctx: &'a Context,
    pub guild_id: &'a GuildId,
}

impl<'a> Discord<'a> {
    pub fn new(ctx: &'a Context, guild_id: &'a GuildId) -> Self {
        Discord { ctx, guild_id }
    }
}