use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Schedule {
    pub content: String,
    pub deadline: NaiveDate
}