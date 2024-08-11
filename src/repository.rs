use crate::{db::Queryer, entities::Url, Error};

#[derive(Debug)]
pub struct Repository {}

impl Repository {
    pub fn new() -> Repository {
        Repository {}
    }

    pub async fn create_url(&self, db: impl Queryer<'_>, url: &Url) -> Result<Url, Error> {
        const QUERY: &str = "INSERT INTO urls
            (id, url)
            VALUES ($1, $2)
            ON CONFLICT(url) DO UPDATE SET url=EXCLUDED.url RETURNING id";
        let row = sqlx::query_as(QUERY)
            .bind(&url.id)
            .bind(&url.url)
            .fetch_one(db)
            .await?;
        Ok(row)
    }

    pub async fn get_url_by_id(&self, db: impl Queryer<'_>, id: &str) -> Result<Url, Error> {
        const QUERY: &str = "SELECT id, url FROM urls WHERE id = $1";
        let url = sqlx::query_as(QUERY).bind(id).fetch_one(db).await?;
        Ok(url)
    }
}
