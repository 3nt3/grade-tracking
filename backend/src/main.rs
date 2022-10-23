#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate sqlx;

#[macro_use]
extern crate tokio;

use sqlx::{postgres::PgQueryResult, query, PgConnection, PgPool, Pool, Postgres};

mod db;
mod models;
mod routes;

#[rocket::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    let pool = PgPool::connect(&std::env::var("DATABASE_URL").unwrap()).await?;

    create_tables(&pool).await?;
    dbg!(list_tables(&pool).await?);

    // query!("select * from grades")

    rocket::build()
        .mount("/", routes![routes::get_grades])
        .manage(pool)
        .launch()
        .await?;

    Ok(())
}

async fn create_tables(conn: &Pool<Postgres>) -> Result<PgQueryResult, sqlx::Error> {
    query!(
        "create table if not exists grades (
            id serial primary key,
            subject text not null,
            points int not null,
            created_at timestamptz not null,
            scored_at timestamptz not null);"
    )
    .execute(conn)
    .await
}

async fn list_tables(conn: &Pool<Postgres>) -> Result<Vec<String>, sqlx::Error> {
    let records = query!("select tablename from pg_catalog.pg_tables where schemaname = 'public'")
        .fetch_all(conn)
        .await?;
    Ok(records
        .iter()
        .map(|table| table.tablename.clone().unwrap_or_default())
        .collect())
}
