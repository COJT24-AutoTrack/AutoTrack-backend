use serde::{Deserialize, Serialize};
use time::{OffsetDateTime, Date};

#[derive(Debug, Serialize, Deserialize)]
pub struct Maintenance {
    pub maint_id: i32,
    pub car_id: i32,
    pub maint_type: String,
    pub maint_date: Date,
    pub maint_description: String,
    pub created_at: Option<OffsetDateTime>,
    pub updated_at: Option<OffsetDateTime>,
}
