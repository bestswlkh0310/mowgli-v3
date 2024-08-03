use serenity::all::{CommandDataOptionValue, CommandInteraction, CreateEmbed, CreateInteractionResponseMessage, InteractionResponseFlags};
use serenity::{async_trait, Error};
use crate::commands::CommandTrait;
use crate::database::meta_data_repo::MetaDataRepo;
use crate::global::discord::Discord;
use crate::util::colour::GREEN;

pub struct SettingMainChannelCommand;

#[async_trait]
impl CommandTrait for SettingMainChannelCommand {
    async fn run(discord: &Discord, command: &CommandInteraction) -> serenity::Result<Option<CreateInteractionResponseMessage>> {
        let option = command.data.options.first().ok_or_else(|| Error::Other("invalid option"))?;
        if option.name != "main-channel" {
            Err(Error::Other("에러"))?
        }
        let command = match &option.value {
            CommandDataOptionValue::SubCommand(value) => value,
            _ => Err(Error::Other("에러"))?
        }.first().ok_or_else(|| Error::Other("에러"))?;

        if command.name != "channel" {
            Err(Error::Other("에러"))?;
        }

        let channel_id = match command.value {
            CommandDataOptionValue::Channel(value) => value,
            _ => Err(Error::Other("에러"))?
        };

        let meta_data_repo = MetaDataRepo::new(discord);
        meta_data_repo.edit_main_channel(channel_id.get()).await?;

        let create_embed = CreateEmbed::new()
            .description("### 메인 채널 등록 성공! 😍")
            .color(GREEN);

        Ok(Some(CreateInteractionResponseMessage::new()
            .flags(InteractionResponseFlags::EPHEMERAL)
            .add_embed(create_embed)))
    }
}