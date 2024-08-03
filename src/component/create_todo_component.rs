use std::str::FromStr;

use chrono::NaiveDate;
use serenity::{async_trait, Error};
use serenity::all::{ComponentInteraction, CreateInputText, CreateInteractionResponse, CreateInteractionResponseMessage, CreateQuickModal, InputTextStyle, InteractionResponseFlags};
use serenity::builder::CreateEmbed;

use crate::component::ComponentTrait;
use crate::database::todo_repo::TodoRepo;
use crate::entity::team::Team;
use crate::entity::todo::{Todo, TodoContent};
use crate::global::discord::Discord;
use crate::util::colour::GREEN;
use crate::util::create_embed_extension::CreateEmbedExtension;

pub struct CreateTodoComponent;

#[async_trait]
impl ComponentTrait for CreateTodoComponent {
    async fn run(discord: &Discord, component: &ComponentInteraction) -> serenity::Result<Option<CreateInteractionResponseMessage>> {
        let team_name = &component.data.custom_id;
        let modal = CreateQuickModal::new("todo 추가")
            .field(
                CreateInputText::new(InputTextStyle::Short, "투두", "content")
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
        let response = component.quick_modal(discord.ctx, modal).await?.ok_or_else(|| Error::Other("response is None"))?;
        let inputs = &response.inputs;
        let (content, deadline) = (&inputs[0], &inputs[1]);
        let d: Vec<&str> = deadline.split("/").collect();
        println!("{:?}", d);
        if d.iter().count() != 2 {
            let create_embed = CreateEmbed::error_create_embed("마감일을 제대로 입력해주세요. \nex. 3월 2일 -> 3/2".to_string());
            let message = CreateInteractionResponseMessage::new()
                .add_embed(create_embed)
                .flags(InteractionResponseFlags::EPHEMERAL);
            let builder = CreateInteractionResponse::Message(message);
            if let Err(why) = response.interaction.create_response(&discord.ctx.http, builder).await {
                println!("{}.0 Err - {}", file!(), why);
            };
            return Ok(None)
        }

        let m = u32::from_str(d[0]).map_err(|_| Error::Other("month 파싱 실패"))?;
        let d = u32::from_str(d[1]).map_err(|_| Error::Other("day 파싱 실패"))?;

        println!("{}, {}", m, d);

        let todo_repo = TodoRepo::new(discord);
        let todo = Todo {
            team: Team { name: team_name.clone() },
            todo: TodoContent {
                content: content.clone(),
                deadline: NaiveDate::from_ymd_opt(2024, m, d).ok_or_else(|| Error::Other("NaiveDate 파싱 실패"))?,
            },
        };
        todo_repo.create_todo(&todo).await?;

        let create_embed = CreateEmbed::new()
            .title("투두추가 성공")
            .color(GREEN)
            .description(format!("{}까지 {}", deadline, content));

        let message = CreateInteractionResponseMessage::new()
            .flags(InteractionResponseFlags::EPHEMERAL)
            .add_embed(create_embed.clone());

        let builder = CreateInteractionResponse::Message(message);

        if let Err(why) = response.interaction.create_response(&discord.ctx.http, builder).await {
            println!("{} Err.1 - {}", file!(), why);
        };

        Ok(None)
    }
}