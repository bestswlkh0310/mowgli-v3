use serenity::all::{Context, GuildId, Member};

pub async fn find_members(ctx: &Context, guild_id: GuildId) -> Result<Vec<Member>, String> {
    match ctx.http.get_guild_members(guild_id, None, None).await {
        Ok(members) => Ok(members),
        Err(why) => Err(format!("멤버 불러오기 실패 - {}", why)),
    }
}

pub async fn find_member<'a>(username: &str, members: &'a [Member]) -> Result<&'a Member, String> {
    match members.iter()
        .find(|member| member.user.name == username) {
        Some(value) => Ok(value),
        None => Err(String::from("멤버 찾기 실패"))
    }
}