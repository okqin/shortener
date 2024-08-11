use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct Url {
    #[sqlx(default)]
    pub id: String,

    #[sqlx(default)]
    pub url: String,
}
