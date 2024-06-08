use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub user_id: Option<i32>,
    pub user_email: String,
    pub user_name: String,
    pub user_password: String,
    pub created_at: OffsetDateTime,
    pub updated_at: Option<OffsetDateTime>,
}
