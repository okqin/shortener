use crate::config;
use crate::Error;
use sqlx::{postgres::PgPoolOptions, Executor, PgPool};
use std::time::Duration;

pub type DB = PgPool;

pub trait Queryer<'c>: Executor<'c, Database = sqlx::Postgres> {}
impl<'c> Queryer<'c> for &PgPool {}

pub async fn connect(database: &config::Database) -> Result<DB, Error> {
    let db_pool = PgPoolOptions::new()
        .max_connections(database.pool_size)
        .max_lifetime(Duration::from_secs(30 * 60))
        .connect(&database.url)
        .await?;
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS urls (id CHAR(6) PRIMARY KEY, url TEXT NOT NULL UNIQUE)",
    )
    .execute(&db_pool)
    .await?;
    Ok(db_pool)
}
