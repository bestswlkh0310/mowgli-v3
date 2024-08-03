use serenity::all::{CommandInteraction, CreateEmbed, CreateInteractionResponseMessage, InteractionResponseFlags};
use serenity::{async_trait, Error};
use crate::commands::CommandTrait;
use crate::database::meta_data_repo::MetaDataRepo;
use crate::global::discord::{Discord, Guild};
use crate::util::colour::GREEN;

pub struct RemindCommand;

#[async_trait]
impl CommandTrait for RemindCommand {
    async fn run(discord: &Discord, _command: &CommandInteraction) -> serenity::Result<Option<CreateInteractionResponseMessage>> {
        let meta_data_repo = MetaDataRepo::new(Guild::from(discord));
        let main_channel_id = meta_data_repo.get_main_channel().await?;
        let channels = discord.guild_id.channels(&discord.ctx.http).await?;
        let channel = channels.iter().find(|(id, _)| id.get() == main_channel_id).ok_or_else(|| Error::Other("메인 채널을 찾을 수 없습니다"))?;
        channel.1.say(&discord.ctx.http, "이야호 ㅋ 테스트 입니다 @everyone").await?;
        let create_embed = CreateEmbed::new()
            .title("리마인드 성공")
            .color(GREEN);
        Ok(Some(CreateInteractionResponseMessage::new()
            .flags(InteractionResponseFlags::EPHEMERAL)
            .add_embed(create_embed)))
    }
}