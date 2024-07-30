use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Serialize, Deserialize)]
pub struct Tuning {
    pub tuning_id: Option<i32>,
    pub car_id: i32,
    pub tuning_name: String,
    pub tuning_date: OffsetDateTime,
    pub tuning_description: String,
    pub created_at: Option<OffsetDateTime>,
    pub updated_at: Option<OffsetDateTime>,
}
