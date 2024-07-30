use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Serialize, Deserialize)]
pub struct Accident {
    pub accident_id: Option<i32>,
    pub car_id: i32,
    pub accident_date: NaiveDate,
    pub accident_description: String,
    pub created_at: Option<OffsetDateTime>,
    pub updated_at: Option<OffsetDateTime>,
}
