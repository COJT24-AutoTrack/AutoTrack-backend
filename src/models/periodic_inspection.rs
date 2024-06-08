use serde::{Deserialize, Serialize};
use time::{OffsetDateTime, Date};

#[derive(Debug, Serialize, Deserialize)]
pub struct PeriodicInspection {
    pub pi_id: i32,
    pub car_id: i32,
    pub pi_name: String,
    pub pi_date: Date,
    pub pi_nextdate: Date,
    pub created_at: Option<OffsetDateTime>,
    pub updated_at: Option<OffsetDateTime>,
}