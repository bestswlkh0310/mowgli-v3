use std::str::FromStr;

use chrono::NaiveDate;
use serenity::{async_trait, Error};
use serenity::all::{ComponentInteraction, CreateInputText, CreateQuickModal, InputTextStyle};
use serenity::builder::CreateEmbed;

use crate::component::ComponentTrait;
use crate::database::todo_repo::TodoRepo;
use crate::entity::team::Team;
use crate::entity::todo::{Todo, TodoContent};
use crate::global::discord::Discord;
use crate::util::create_interaction_response_extension::create_response;
use crate::util::create_embed_extension::CreateEmbedExtension;

pub struct CreateTodoComponent;

#[async_trait]
impl ComponentTrait for CreateTodoComponent {
    async fn run(discord: &Discord, component: &ComponentInteraction) -> serenity::Result<Option<CreateEmbed>> {
        let team_name = &component.data.custom_id.to_lowercase();
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

        let response = match component.quick_modal(discord.ctx, modal).await {
            Ok(Some(res)) => res,
            Err(why) => return Err(why),
            _ => return Err(Error::Other("response is None"))
        };
        let inputs = &response.inputs;
        let (content, deadline) = (&inputs[0], &inputs[1]);
        let d: Vec<&str> = deadline.split("/").collect();
        println!("{:?}", d);
        if d.iter().count() != 2 {
            let create_embed = CreateEmbed::make_create_embed("마감일을 제대로 입력해주세요. \nex. 3월 2일 -> 3/2".to_string());
            let builder = create_response(create_embed);
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
            .description(format!("{}까지 {} 화이팅!", deadline, content));

        let builder = create_response(create_embed);
        if let Err(why) = response.interaction.create_response(&discord.ctx.http, builder).await {
            println!("{} Err.1 - {}", file!(), why);
        };

        Ok(None)
    }
}