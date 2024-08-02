use std::str::FromStr;
use chrono::NaiveDate;
use serenity::all::{ComponentInteraction, Context, CreateInputText, CreateInteractionResponse, CreateInteractionResponseMessage, CreateQuickModal, InputTextStyle};
use serenity::{async_trait, Error};
use serenity::builder::CreateEmbed;
use crate::component::ComponentTrait;
use crate::database::database::{Database, DatabaseTrait};
use crate::database::todo::TodoRepo;
use crate::entity::team::Team;
use crate::entity::todo::{Todo, TodoContent};

pub struct CreateTodoComponent;

#[async_trait]
impl ComponentTrait for CreateTodoComponent {
    async fn run(ctx: &Context, component: &ComponentInteraction) -> serenity::Result<()> {
        let guild_id = component.guild_id
            .ok_or_else(|| Error::Other("guild id가 없습니다"))?;
        let team_name = &component.data.custom_id.to_lowercase();
        let modal = CreateQuickModal::new("할일추가")
            .field(
                CreateInputText::new(InputTextStyle::Short, "할일", "content")
                    .placeholder("Auth 기능 구현")
                    .min_length(1)
                    .max_length(300)
            )
            .field(
                CreateInputText::new(InputTextStyle::Short, "마감기한", "deadline")
                    .placeholder("ex. 3월 2일 -> 3/2")
                    .min_length(3)
                    .max_length(5)
            );

        let response = match component.quick_modal(ctx, modal).await {
            Ok(Some(res)) => res,
            Err(why) => return Err(why),
            _ => return Err(Error::Other("response is None"))
        };
        let inputs = &response.inputs;
        let (content, deadline) = (&inputs[0], &inputs[1]);
        let d: Vec<&str> = deadline.split("/").collect();
        println!("{:?}", d);
        if d.iter().count() != 2 {
            let create_embed = CreateEmbed::new()
                .title("마감일을 제대로 입력해주세요")
                .description("ex. 3월 2일 -> 3/2");

            let data = CreateInteractionResponseMessage::new()
                .add_embed(create_embed);
            let builder = CreateInteractionResponse::Message(data);
            response.interaction.create_response(&ctx.http, builder).await?;
            return Err(Error::Other("잘못된 deadline"));
        }

        let m = u32::from_str(d[0]).map_err(|_| Error::Other("month 파싱 실패"))?;
        let d = u32::from_str(d[1]).map_err(|_| Error::Other("day 파싱 실패"))?;

        println!("{}, {}", m, d);

        let entity = Database.get_entity(&ctx.http, &guild_id).await?;
        let mut todo_repo = TodoRepo::new(entity.clone());
        let todo = Todo {
            team: Team { name: team_name.clone() },
            todo: TodoContent {
                content: content.clone(),
                deadline: NaiveDate::from_ymd_opt(2024, m, d).ok_or_else(|| Error::Other("NaiveDate 파싱 실패"))?,
            },
        };
        todo_repo.create_todo(todo)?;
        Database.edit_entity(&ctx.http, &guild_id, &todo_repo.entity).await?;

        let create_embed = CreateEmbed::new()
            .title("할일추가 성공")
            .description(format!("{}까지 {}. 화이팅!", deadline, content));

        let data = CreateInteractionResponseMessage::new()
            .add_embed(create_embed);
        let builder = CreateInteractionResponse::Message(data);
        response.interaction.create_response(&ctx.http, builder).await?;
        Ok(())
    }
}
/*
ComponentInteraction {
    id: InteractionId(
        1268753631818158151,
    ),
    application_id: ApplicationId(
        1268146621896720464,
    ),
    data: ComponentInteractionData {
        custom_id: "iOS",
        kind: Button,
    },
    guild_id: Some(
        GuildId(
            1243554058221129838,
        ),
    ),
    channel: Some(
        PartialChannel {
            id: ChannelId(
                1243554058808197193,
            ),
            name: Some(
                "일반",
            ),
            kind: Text,
            permissions: Some(
                Permissions(
                    985162418487295,
                ),
            ),
            thread_metadata: None,
            parent_id: Some(
                ChannelId(
                    1243554058808197191,
                ),
            ),
        },
    ),
    channel_id: ChannelId(
        1243554058808197193,
    ),
    member: Some(
        Member {
            user: User {
                id: UserId(
                    805819594253533245,
                ),
                name: "hhhello0507",
                discriminator: None,
                global_name: Some(
                    "이강현",
                ),
                avatar: Some(
                    "f80ee2bea03fd808e067f6820148c994",
                ),
                bot: false,
                system: false,
                mfa_enabled: false,
                banner: None,
                accent_colour: None,
                locale: None,
                verified: None,
                email: None,
                flags: UserPublicFlags(
                    0,
                ),
                premium_type: None,
                public_flags: Some(
                    UserPublicFlags(
                        0,
                    ),
                ),
                member: None,
            },
            nick: None,
            avatar: None,
            roles: [],
            joined_at: Some(
                Timestamp(
                    2024-05-24T13:19:51.645Z,
                ),
            ),
            premium_since: None,
            deaf: false,
            mute: false,
            flags: GuildMemberFlags(
                0,
            ),
            pending: false,
            permissions: Some(
                Permissions(
                    985162418487295,
                ),
            ),
            communication_disabled_until: None,
            guild_id: GuildId(
                1243554058221129838,
            ),
            unusual_dm_activity_until: None,
        },
    ),
    user: User {
        id: UserId(
            805819594253533245,
        ),
        name: "hhhello0507",
        discriminator: None,
        global_name: Some(
            "이강현",
        ),
        avatar: Some(
            "f80ee2bea03fd808e067f6820148c994",
        ),
        bot: false,
        system: false,
        mfa_enabled: false,
        banner: None,
        accent_colour: None,
        locale: None,
        verified: None,
        email: None,
        flags: UserPublicFlags(
            0,
        ),
        premium_type: None,
        public_flags: Some(
            UserPublicFlags(
                0,
            ),
        ),
        member: None,
    },
    token: "aW50ZXJhY3Rpb246MTI2ODc1MzYzMTgxODE1ODE1MTp5QkRqdlZvZWZFTHFzSXp3Y3JQcUpZTVdmT3JCTGlGQ0s2NjVseFNnNUN2cW1pRGVad0RuTFZ2cllncE51Uk9zTWFpb2I5dVhJRnpMcG1ZbEN1TnZ6RDI3NksxaHJCMHhXc0VzTVVodWhuMElFaWd5Q1RtaW9CcHdKSG9Jb2JTeQ",
    version: 1,
    message: Message {
        id: MessageId(
            1268742804918501459,
        ),
        channel_id: ChannelId(
            1243554058808197193,
        ),
        author: User {
            id: UserId(
                1268146621896720464,
            ),
            name: "모글리v3",
            discriminator: Some(
                5773,
            ),
            global_name: None,
            avatar: Some(
                "39aea05846c8fb7b6f019c2c2b2d5e60",
            ),
            bot: true,
            system: false,
            mfa_enabled: false,
            banner: None,
            accent_colour: None,
            locale: None,
            verified: None,
            email: None,
            flags: UserPublicFlags(
                0,
            ),
            premium_type: None,
            public_flags: Some(
                UserPublicFlags(
                    0,
                ),
            ),
            member: None,
        },
        content: "팀을 알려주세요!",
        timestamp: Timestamp(
            2024-08-02T01:30:56.794Z,
        ),
        edited_timestamp: None,
        tts: false,
        mention_everyone: false,
        mentions: [],
        mention_roles: [],
        mention_channels: [],
        attachments: [],
        embeds: [],
        reactions: [],
        nonce: None,
        pinned: false,
        webhook_id: Some(
            WebhookId(
                1268146621896720464,
            ),
        ),
        kind: ChatInputCommand,
        activity: None,
        application: None,
        application_id: Some(
            ApplicationId(
                1268146621896720464,
            ),
        ),
        message_reference: None,
        flags: Some(
            MessageFlags(
                0,
            ),
        ),
        referenced_message: None,
        interaction: Some(
            MessageInteraction {
                id: InteractionId(
                    1268742800758018139,
                ),
                kind: Command,
                name: "할일추가",
                user: User {
                    id: UserId(
                        805819594253533245,
                    ),
                    name: "hhhello0507",
                    discriminator: None,
                    global_name: Some(
                        "이강현",
                    ),
                    avatar: Some(
                        "f80ee2bea03fd808e067f6820148c994",
                    ),
                    bot: false,
                    system: false,
                    mfa_enabled: false,
                    banner: None,
                    accent_colour: None,
                    locale: None,
                    verified: None,
                    email: None,
                    flags: UserPublicFlags(
                        0,
                    ),
                    premium_type: None,
                    public_flags: Some(
                        UserPublicFlags(
                            0,
                        ),
                    ),
                    member: None,
                },
                member: None,
            },
        ),
        thread: None,
        components: [
            ActionRow {
                kind: ActionRow,
                components: [
                    Button(
                        Button {
                            kind: Button,
                            data: NonLink {
                                custom_id: "iOS",
                                style: Secondary,
                            },
                            label: Some(
                                "iOS",
                            ),
                            emoji: None,
                            disabled: false,
                        },
                    ),
                    Button(
                        Button {
                            kind: Button,
                            data: NonLink {
                                custom_id: "Android",
                                style: Secondary,
                            },
                            label: Some(
                                "Android",
                            ),
                            emoji: None,
                            disabled: false,
                        },
                    ),
                    Button(
                        Button {
                            kind: Button,
                            data: NonLink {
                                custom_id: "Web",
                                style: Secondary,
                            },
                            label: Some(
                                "Web",
                            ),
                            emoji: None,
                            disabled: false,
                        },
                    ),
                    Button(
                        Button {
                            kind: Button,
                            data: NonLink {
                                custom_id: "Server",
                                style: Secondary,
                            },
                            label: Some(
                                "Server",
                            ),
                            emoji: None,
                            disabled: false,
                        },
                    ),
                ],
            },
        ],
        sticker_items: [],
        position: Some(
            0,
        ),
        role_subscription_data: None,
        guild_id: None,
        member: None,
        poll: None,
    },
    app_permissions: Some(
        Permissions(
            985162418487295,
        ),
    ),
    locale: "ko",
    guild_locale: Some(
        "en-US",
    ),
    entitlements: [],
}

 */