use axum::body::Body;
use axum::http::{Response, StatusCode};
use axum::response::IntoResponse;
use axum::Json;
use miniflux_api::ApiError;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ErrorBody {
    error: String,
}

impl ErrorBody {
    fn new(error: String) -> Self {
        ErrorBody { error }
    }
}

#[derive(Error, Debug)]
pub enum RoutingError {
    #[error("Miniflux Error")]
    MiniFluxError(#[from] ApiError),
}

pub type Result<T, E = RoutingError> = std::result::Result<T, E>;

impl IntoResponse for RoutingError {
    fn into_response(self) -> Response<Body> {
        let (status_code, error) = match self {
            Self::MiniFluxError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Error".to_string(),
            ),
        };

        let error_body = ErrorBody::new(error.to_string());
        (status_code, Json(error_body)).into_response()
    }
}
