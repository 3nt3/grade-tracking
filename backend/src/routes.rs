use std::io::Cursor;

use chrono::{DateTime, NaiveDateTime, Utc};
use rocket::{http::Status, response::Responder, serde::json::Json, Response, State};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Pool, Postgres};

use crate::models::Grade;

use super::models;

#[path = "db.rs"]
mod db;

#[get("/grades")]
pub async fn get_grades(pool: &State<Pool<Postgres>>) -> Json<Vec<models::Grade>> {
    Json(db::get_grades(&pool).await.unwrap_or(vec![]))
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GradeReq {
    subject: String,
    points: i32,
    scored_at: chrono::NaiveDate,
}

impl Into<Grade> for GradeReq {
    fn into(self) -> Grade {
        Grade {
            subject: self.subject,
            points: self.points,
            scored_at: DateTime::<Utc>::from_utc(self.scored_at.and_hms(0, 0, 0), Utc),
            ..Default::default()
        }
    }
}

#[post("/grades", data = "<body>")]
pub async fn create_grade(
    pool: &State<PgPool>,
    body: Json<GradeReq>,
) -> Result<Json<models::Grade>, String> {
    dbg!(&body.0);
    let res = db::create_grade(&body.0.into(), pool).await;

    match res {
        Ok(grade) => Ok(Json(grade)),
        Err(why) => Err(why.to_string()),
    }
}
