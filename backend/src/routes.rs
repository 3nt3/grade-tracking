use rocket::{serde::json::Json, State};
use sqlx::{Pool, Postgres};

use super::models;

#[path = "db.rs"]
mod db;

#[get("/get-grades")]
pub async fn get_grades(pool: &State<Pool<Postgres>>) -> Json<Vec<models::Grade>> {
    Json(db::get_grades(&pool).await.unwrap_or(vec![]))
}
