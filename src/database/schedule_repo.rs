use crate::database::database::{Database, DatabaseTrait};
use crate::entity::schedule::Schedule;
use crate::global::discord::Guild;

pub struct ScheduleRepo {
    guild: Guild,
}

impl ScheduleRepo {
    async fn create_schedule(&self, schedule: Schedule) -> serenity::Result<()> {
        let mut entity = Database.get_entity(&self.guild).await?;
        entity.schedules.push(schedule);
        Database.edit_entity(&self.guild, &entity).await
    }

    async fn get_schedule(&self) -> serenity::Result<Vec<Schedule>> {
        let entity = Database.get_entity(&self.guild).await?;
        Ok(entity.schedules)
    }
}