use serde::{Deserialize, Serialize};
use time::{OffsetDateTime, Date};

#[derive(Debug, Serialize, Deserialize)]
pub struct Accident {
    pub accident_id: i32,
    pub car_id: i32,
    pub accident_date: Date,
    pub accident_description: String,
    pub created_at: Option<OffsetDateTime>,
    pub updated_at: Option<OffsetDateTime>,
}
