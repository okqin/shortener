use crate::api::{ApiContext, Result};
use axum::{
    extract::{Path, State},
    response::Redirect,
    routing::{get, post},
    Json, Router,
};
use tracing::instrument;

use super::model::{ShortenRequest, ShortenResponse};

pub fn router() -> Router<ApiContext> {
    Router::new()
        .route("/", post(create_short_url))
        .route("/:id", get(get_original_url))
}

#[instrument(skip(ctx))]
async fn create_short_url(
    State(ctx): State<ApiContext>,
    Json(url): Json<ShortenRequest>,
) -> Result<Json<ShortenResponse>> {
    let url = ctx.service.create_short_url(&url.url).await?;
    Ok(Json(url.into()))
}

#[instrument(skip(ctx))]
async fn get_original_url(
    State(ctx): State<ApiContext>,
    Path(id): Path<String>,
) -> Result<Redirect> {
    let url = ctx.service.get_original_url(&id).await?;
    Ok(url.into())
}
