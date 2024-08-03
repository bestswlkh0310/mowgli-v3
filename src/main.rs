use std::io::Error;
use std::time::{SystemTime, UNIX_EPOCH};
use serenity::{
    all::Message,
    async_trait,
    gateway::ActivityData,
    model::{
        application::{Command, Interaction},
        gateway::Ready,
    },
    prelude::*,
    utils::MessageBuilder,
};
use serenity::all::{CommandInteraction, CommandOptionType, ComponentInteraction, CreateCommand, CreateCommandOption};
use serenity::builder::CreateInteractionResponse;
use crate::commands::CommandTrait;
use crate::commands::create_todo_command::AskTeamCommand;
use crate::commands::force_import_db_command::ForceImportDBCommand;
use crate::commands::not_found_command::NotFoundCommand;
use crate::commands::remind_command::RemindCommand;
use crate::commands::reset_db_command::ResetDBCommand;
use crate::commands::reset_todos_command::ResetTodosCommand;
use crate::commands::setting_main_channel_command::SettingMainChannelCommand;
use crate::commands::show_all_todos_command::ShowAllTodosCommand;
use crate::component::ComponentTrait;
use crate::component::create_todo_component::CreateTodoComponent;
use crate::component::show_todos_component::ShowTodosComponent;
use crate::component::not_found::NotFountComponent;
use crate::config::config::Config;
use crate::global::discord::Discord;
use crate::util::create_embed_extension::{ResultCreateEmbed};

mod commands;
mod database;
mod util;
mod entity;
mod config;
mod component;
mod global;

struct Handler;

// 욕설 리스트
static ARR: &[&str] = &[
    "ㅅㅂ",
    "시발",
    "병신",
    "ㅂㅅ",
    "장애",
    "새끼",
    "어쩌라고",
    "어쩔",
    "저쩔",
    "html은 프로그래밍 언어",
    "java는 프로그래밍 언어",
    "objective-c는 프로그래밍 언어"
];

// impl for interaction_create fn
impl Handler {
    async fn handle_command_interaction(&self, ctx: &Context, command: &CommandInteraction) {
        // create discord
        let guild_id = match command.guild_id.ok_or_else(|| Error::other("guild id를 찾을 수 없습니다")) {
            Ok(v) => v,
            _ => return
        };
        let discord = Discord::new(ctx, &guild_id);

        // handle command
        let data = &command.data;
        let name = data.name.as_str();
        let option = data.options.first().unwrap().name.as_str();
        let result = match name {
            "todo" => match option {
                "show" => AskTeamCommand::run(&discord, command).await,
                "show-all" => ShowAllTodosCommand::run(&discord, command).await,
                "reset" => ResetTodosCommand::run(&discord, command).await,
                "add" => AskTeamCommand::run(&discord, command).await,
                "remind" => RemindCommand::run(&discord, command).await,
                _ => NotFoundCommand::run(&discord, command).await
            },
            "setting" => match option {
                "main-channel" => SettingMainChannelCommand::run(&discord, command).await,
                _ => NotFoundCommand::run(&discord, command).await
            },
            "db" => match option {
                "reset" => ResetDBCommand::run(&discord, command).await,
                "force-import" => ForceImportDBCommand::run(&discord, command).await,
                _ => NotFoundCommand::run(&discord, command).await
            }
            _ => NotFoundCommand::run(&discord, command).await
        };

        // handle result
        if let Some(message) = result.create_embed() {
            let builder = CreateInteractionResponse::Message(message);
            if let Err(why) = command.create_response(&ctx.http, builder).await {
                println!("API resposne 에러 발생 - {}", why);
            };
        }
    }

    async fn handle_component_interaction(&self, ctx: &Context, component: &ComponentInteraction) {

        // validation
        let message_interaction = match &component.message.interaction {
            Some(v) => v,
            _ => {
                print!("message interaction 이 외에 핸들링 필요");
                return;
            }
        };
        // create discord
        let guild_id = match component.guild_id.ok_or_else(|| Error::other("guild id를 찾을 수 없습니다")) {
            Ok(v) => v,
            _ => return
        };
        let discord = Discord::new(ctx, &guild_id);

        // handle message interaction
        let interaction_name = message_interaction.name.as_str();
        let result = match interaction_name {
            "todo add" => CreateTodoComponent::run(&discord, component).await,
            "todo show" => ShowTodosComponent::run(&discord, component).await,
            _ => NotFountComponent::run(&discord, component).await
        };

        // handle result
        if let Some(message) = result.create_embed() {
            let builder = CreateInteractionResponse::Message(message);
            if let Err(why) = component.create_response(&ctx.http, builder).await {
                println!("API resposne 에러 발생 - {}", why);
            };
        }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let content = msg.content;
        if ARR.iter().any(|&i| content.contains(i)) {
            let response = MessageBuilder::new()
                .push(format!("{}님 ", msg.author.mention()))
                .push_bold_safe("올바른 언어 습관")
                .push("을 들입시다")
                .build();
            _ = msg.channel_id.say(&ctx.http, &response).await;
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} 봇 실행 완료!", ready.user.name);

        // set activity
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        ctx.set_activity(Some(ActivityData::playing(format!("{}", now))));

        // register command
        Command::set_global_commands(&ctx.http, vec![
            CreateCommand::new("todo")
                .description("투두~")
                .add_option(
                    CreateCommandOption::new(CommandOptionType::SubCommand, "add", "투두 추가")
                )
                .add_option(
                    CreateCommandOption::new(CommandOptionType::SubCommand, "reset", "투두 초기화")
                )
                .add_option(
                    CreateCommandOption::new(CommandOptionType::SubCommand, "show", "투두 확인")
                )
                .add_option(
                    CreateCommandOption::new(CommandOptionType::SubCommand, "show-all", "전체 투두 확인")
                )
                .add_option(
                    CreateCommandOption::new(CommandOptionType::SubCommand, "remind", "리마인드")
                ),
            CreateCommand::new("setting")
                .description("설정~")
                .add_option(
                    CreateCommandOption::new(CommandOptionType::SubCommand, "main-channel", "메인 채널 설정")
                        .add_sub_option(
                            CreateCommandOption::new(CommandOptionType::Channel, "channel", "메인 채널")
                                .required(true)
                        )
                ),
            CreateCommand::new("db")
                .description("데이터베이스 설정")
                .add_option(
                    CreateCommandOption::new(CommandOptionType::SubCommand, "reset", "DB 전체 초기화")
                )
                .add_option(
                    CreateCommandOption::new(CommandOptionType::SubCommand, "force-import", "DB 강제 불러오기")
                        .add_sub_option(
                            CreateCommandOption::new(CommandOptionType::String, "json", "json raw value")
                                .required(true)
                        )
                )
        ])
            .await
            .expect("명령 생성에 실패했습니다.");
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        match interaction {
            Interaction::Command(command) => self.handle_command_interaction(&ctx, &command).await,
            Interaction::Component(component) => self.handle_component_interaction(&ctx, &component).await,
            _ => return,
        };
    }
}

#[tokio::main]
async fn main() {
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::DIRECT_MESSAGES;

    let config = Config::new();
    let mut client = Client::builder(config.discord_bot_token, intents)
        .event_handler(Handler)
        .await
        .expect("클라이언트 생성에 실패했습니다.");

    if let Err(why) = client.start().await {
        println!("클라이언트 오류가 발생했습니다: {why}");
    }
}