use serde::{Deserialize, Serialize};
use time::{OffsetDateTime, Date};

#[derive(Debug, Serialize, Deserialize)]
pub struct Tuning {
    pub tuning_id: Option<i32>,
    pub car_id: i32,
    pub tuning_name: String,
    pub tuning_date: Date,
    pub tuning_description: String,
    pub created_at: Option<OffsetDateTime>,
    pub updated_at: Option<OffsetDateTime>,
}