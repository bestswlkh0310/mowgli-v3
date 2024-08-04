use std::str::FromStr;
use std::sync::Arc;
use chrono::Utc;
use serenity::all::{Http};
use tokio::time::{Instant, sleep_until};
use tokio_cron_scheduler::{Job, JobBuilder, JobScheduler, JobSchedulerError};
use crate::database::meta_data_repo::MetaDataRepo;
use crate::database::team_repo::TeamRepo;
use crate::database::todo_repo::TodoRepo;
use crate::entity::todo::{Todo, VecTodoExtension};
use crate::global::discord::Guild;

pub async fn schedule_task(http: Arc<Http>) -> Result<Job, JobSchedulerError> {
    JobBuilder::new()
        .with_timezone(chrono_tz::Asia::Seoul)
        .with_cron_job_type()
        .with_schedule("0 20 8 * * Mon *") // 매주 월요일 아침 8시 20분 ~
        .unwrap()
        .with_run_async(
            Box::new(move |uuid, mut l| {
                let http = http.clone();
                Box::pin(async move {
                    let next_tick = l.next_tick_for_job(uuid).await;
                    match next_tick {
                        Ok(Some(_)) => {
                            _ = g(&http).await;
                        }
                        _ => println!("Could not get next tick for 7s job"),
                    }
                })
            })
        )
        .build()
}

async fn g(http: &Arc<Http>) -> serenity::Result<()> {
    let guilds = http.get_guilds(None, None).await?;

    for guild in guilds {
        let guild_id = guild.id;
        let guild = Guild { http: http.clone(), guild_id };
        let meta_data_repo = MetaDataRepo::new(guild.clone());
        let channel_id = match meta_data_repo.get_main_channel().await {
            Ok(v) => v,
            Err(why) => {
                println!("channel id 불러오기 실패{}", why);
                continue
            },
        };
        let channels = match guild_id.channels(&http).await {
            Ok(v) => v,
            Err(why) => {
                println!("channels 불러오기 실패{}", why);
                continue
            },
        };

        let team_repo = TeamRepo::new(guild.clone());
        let todo_repo = TodoRepo::new(guild.clone());

        let teams = team_repo.get_teams().await?;
        let todos = todo_repo.get_todos().await?;
        let mut message = String::new();
        message.push_str("# 새로운 스프린트가 시작됐습니다! @everyone\n");
        for team in teams {
            let todos: Vec<Todo> = todos.iter().filter(|todo| todo.team.name == team.name).cloned().collect();
            let mut m = todos.message(&team.name);
            m.push_str("\n\n");
            message.push_str(m.as_str());
        }

        if let Some(channel) = channels.iter().find(|(id, _)| id.get() == channel_id) {
            channel.0.say(&http, message).await?;
        }
    }

    Ok(())
}