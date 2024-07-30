use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Serialize, Deserialize)]
pub struct FuelEfficiency {
    pub fe_id: Option<i32>,
    pub car_id: i32,
    pub fe_date: NaiveDate,
    pub fe_amount: f32,
    pub fe_unitprice: f32,
    pub fe_mileage: i32,
    pub created_at: Option<OffsetDateTime>,
    pub updated_at: Option<OffsetDateTime>,
}
