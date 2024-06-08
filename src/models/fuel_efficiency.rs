use serde::{Deserialize, Serialize};
use time::{OffsetDateTime, Date};

#[derive(Debug, Serialize, Deserialize)]
pub struct FuelEfficiency {
    pub fe_id: Option<i32>,
    pub car_id: i32,
    pub fe_date: Date,
    pub fe_amount: f32,
    pub fe_unitprice: f32,
    pub fe_mileage: i32,
    pub created_at: Option<OffsetDateTime>,
    pub updated_at: Option<OffsetDateTime>,
}
