use sqlx::PgPool;

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
