#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    NotFound(String),

    #[error("{0}")]
    InvalidArgument(String),

    #[error("Internal error")]
    Internal(String),

    #[error("an error occurred with the IO")]
    Io(#[from] std::io::Error),

    #[error("an error occurred with the database")]
    Sqlx(sqlx::Error),

    #[error("an internal server error occurred")]
    Anyhow(#[from] anyhow::Error),
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => Error::NotFound("data not found".to_string()),
            _ => Error::Sqlx(err),
        }
    }
}
