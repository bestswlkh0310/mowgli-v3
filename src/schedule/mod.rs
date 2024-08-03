use std::str::FromStr;
use std::sync::Arc;
use chrono::Utc;
use cron::Schedule;
use serenity::all::{Http};
use tokio::time::{Instant, sleep_until};
use crate::database::meta_data_repo::MetaDataRepo;
use crate::global::discord::Guild;

pub async fn schedule_task(http: &Arc<Http>) {
    let schedule = Schedule::from_str("1 * * * * * *").expect("Failed to parse cron schedule");

    loop {
        let next = schedule.upcoming(Utc).next().expect("Failed to get next schedule");
        let now = Utc::now();
        let duration_until_next = next.signed_duration_since(now).to_std().expect("Failed to convert duration");

        sleep_until(Instant::now() + duration_until_next).await;

        let guilds = match http.get_guilds(None, None).await {
            Ok(v) => v,
            _ => continue
        };

        for guild in guilds {
            let guild_id = guild.id;
            let guild = Guild { http: http.clone(), guild_id };
            let meta_data_repo = MetaDataRepo::new(guild);
            let channel_id = match meta_data_repo.get_main_channel().await {
                Ok(v) => v,
                _ => continue
            };
            let channels = match guild_id.channels(&http).await {
                Ok(v) => v,
                _ => continue
            };
            if let Some(channel) = channels.iter().find(|(id, _)| id.get() == channel_id) {
                if let Err(why) = channel.0.say(&http, "wow").await {
                    println!("{}", why)
                } else {
                    println!("✅  Schedule Success");
                }
            }
        }
    }
}