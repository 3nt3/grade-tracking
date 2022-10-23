use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Grade {
    pub id: i32,
    pub subject: String,
    pub points: i32,
    pub created_at: chrono::DateTime<Utc>,
    pub scored_at: chrono::DateTime<Utc>,
}
