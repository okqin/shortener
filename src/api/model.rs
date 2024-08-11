use crate::entities::Url;
use axum::response::Redirect;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ShortenRequest {
    pub url: String,
}

#[derive(Debug, Serialize)]
pub struct ShortenResponse {
    pub url: String,
}

impl From<Url> for ShortenResponse {
    fn from(url: Url) -> Self {
        let url = format!("http://127.0.0.1:8080/{}", url.id);
        Self { url }
    }
}

impl From<Url> for Redirect {
    fn from(url: Url) -> Self {
        Self::permanent(&url.url)
    }
}
