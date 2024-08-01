use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

use dotenv::dotenv;
use serenity::{
    all::Message,
    async_trait,
    builder::{CreateInteractionResponse, CreateInteractionResponseMessage},
    gateway::ActivityData,
    model::{
        application::{Command, Interaction},
        gateway::Ready,
    },
    prelude::*,
    utils::MessageBuilder,
};
use serenity::all::CreateEmbed;
use crate::commands::CommandTrait;
use crate::commands::get_todos::GetTodosCommand;
use crate::commands::reset_todos::ResetTodosCommand;
use crate::util::error::{ResultCreateEmbed, UnknownCreateEmbed};

mod commands;
mod service;
mod database;
mod util;
mod entity;

struct Handler;

static ARR: &[&str] = &[
    "ㅅㅂ",
    "시발",
    "병신",
    "ㅂㅅ",
    "장애",
    "새끼"
];

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
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        ctx.set_activity(Some(ActivityData::playing(format!("{}", now))));
        Command::set_global_commands(&ctx.http, vec![
            GetTodosCommand::register().await,
            ResetTodosCommand::register().await
        ])
            .await
            .expect("명령 생성에 실패했습니다.");
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        let command = match interaction {
            Interaction::Command(command) => command,
            _ => return
        };

        let command_name = command.data.name.as_str();
        let content = match command_name {
            "할일" => {
                GetTodosCommand::run(&ctx, &command)
                    .await
                    .create_embed()
            }
            "할일초기화" => {
                ResetTodosCommand::run(&ctx, &command)
                    .await
                    .create_embed()
            }
            _ => {
                CreateEmbed::unknown()
            }
        };

        let data = CreateInteractionResponseMessage::new()
            .add_embed(content);
        let builder = CreateInteractionResponse::Message(data);
        if let Err(why) = command.create_response(&ctx.http, builder).await {
            println!("응답할 수 없습니다: {why}");
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("DISCORD_BOT_TOKEN").expect("Discord bot token을 찾을 수 없습니다.");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::DIRECT_MESSAGES;

    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .await
        .expect("클라이언트 생성에 실패했습니다.");

    if let Err(why) = client.start().await {
        println!("클라이언트 오류가 발생했습니다: {why}");
    }
}