// https://github.com/tokio-rs/axum/blob/main/examples/anyhow-error-response/src/main.rs

use axum::{http::StatusCode, response::IntoResponse};
use tracing::error;

#[derive(Debug)]
pub(crate) struct AppError(pub(crate) anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        error!("{:?}", self);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Something went wrong :(".to_string(), // don't expose internals in public error messages. Check logs/tracing
        )
            .into_response()
    }
}
