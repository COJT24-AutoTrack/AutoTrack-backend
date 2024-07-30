use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Serialize, Deserialize)]
pub struct PeriodicInspection {
    pub pi_id: Option<i32>,
    pub car_id: i32,
    pub pi_name: String,
    pub pi_date: DateTime<Local>,
    pub pi_nextdate: DateTime<Local>,
    pub created_at: Option<OffsetDateTime>,
    pub updated_at: Option<OffsetDateTime>,
}
