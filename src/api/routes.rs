use crate::api::{ApiContext, Result};
use axum::{
    extract::{Path, State},
    response::Redirect,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tracing::instrument;

#[derive(Debug, Deserialize)]
struct ShortenRequest {
    url: String,
}

#[derive(Debug, Serialize)]
struct ShortenResponse {
    url: String,
}

pub fn router() -> Router<ApiContext> {
    Router::new()
        .route("/", post(create_short_url))
        .route("/:id", get(get_original_url))
}

#[instrument]
async fn create_short_url(
    State(ctx): State<ApiContext>,
    Json(url): Json<ShortenRequest>,
) -> Result<Json<ShortenResponse>> {
    let id = ctx.service.create_short_url(&url.url).await?;
    let url = format!("http://127.0.0.1:8080/{}", id);
    Ok(Json(ShortenResponse { url }))
}

#[instrument]
async fn get_original_url(
    State(ctx): State<ApiContext>,
    Path(id): Path<String>,
) -> Result<Redirect> {
    let url = ctx.service.get_original_url(&id).await?;
    Ok(Redirect::permanent(&url))
}
