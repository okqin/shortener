use crate::{config::Config, db, service::Service, Error};
use anyhow::Context;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Router,
};
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::info;

mod routes;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Clone)]
pub struct ApiContext {
    pub service: Arc<Service>,
}

pub async fn serve(conf: Config) -> anyhow::Result<()> {
    let listener = TcpListener::bind(&conf.server_addr).await?;
    info!("Server listening on: {}", &conf.server_addr);

    let db = db::connect(&conf.database).await?;
    let service = Arc::new(Service::new(db));
    let ctx = ApiContext { service };

    info!("Connected to database: {}", &conf.database.url);

    let app = api_router().with_state(ctx);

    axum::serve(listener, app)
        .await
        .context("error running HTTP server")
}

fn api_router() -> Router<ApiContext> {
    routes::router()
}

impl Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::InvalidArgument(_) => StatusCode::BAD_REQUEST,
            Self::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Sqlx(_) | Self::Anyhow(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let status_code = self.status_code();
        let body = self.to_string();
        (status_code, body).into_response()
    }
}
