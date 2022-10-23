use sqlx::PgPool;

use crate::models::Grade;

use super::models;

pub async fn get_grades(pool: &PgPool) -> Result<Vec<models::Grade>, sqlx::Error> {
    let records = query!("select * from grades").fetch_all(pool).await?;
    Ok(records
        .iter()
        .map(|r| models::Grade {
            id: r.id,
            subject: r.subject.clone(),
            points: r.points,
            created_at: r.created_at,
            scored_at: r.scored_at,
        })
        .collect())
}

pub async fn create_grade(
    grade: &models::Grade,
    pool: &PgPool,
) -> Result<models::Grade, sqlx::Error> {
    let rec = query!(
        "insert into grades (subject, points, created_at, scored_at) values ($1, $2, now(), $3) returning id",
        grade.subject,
        grade.points,
        grade.scored_at
    )
    .fetch_one(pool).await?;
    let id = rec.id;

    get_grade_by_id(id, pool).await
}

pub async fn get_grade_by_id(id: i32, pool: &PgPool) -> Result<models::Grade, sqlx::Error> {
    query!("select * from grades where id = $1", id)
        .fetch_one(pool)
        .await
        .map(|r| Grade {
            id: r.id,
            subject: r.subject.clone(),
            points: r.points,
            created_at: r.created_at,
            scored_at: r.scored_at,
        })
}
