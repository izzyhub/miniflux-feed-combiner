use thiserror::Error;
use serde::{Deserialize, Serialize};
use axum::http::{header::InvalidHeaderValue, Response, StatusCode};
use axum::body::Body;
use axum::response::IntoResponse;
use axum::Json;
use miniflux_api::ApiError;

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
    #[error("Internal Error")]
    InternalError,
    #[error("Request Error")]
    RequestError,
}

pub type Result<T, E = RoutingError> = std::result::Result<T, E>;

impl IntoResponse for RoutingError {
    fn into_response(self) -> Response<Body> {
        let (status_code, error) = match self {
            Self::MiniFluxError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Error".to_string(),
            ),
            Self::InternalError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Error".to_string(),
            ),
            Self::RequestError => (StatusCode::BAD_REQUEST, "Bad Request".to_string()),
            //Self::NotFound => (StatusCode::NOT_FOUND, "Not found".to_string()),
        };

        let error_body = ErrorBody::new(error.to_string());
        (status_code, Json(error_body)).into_response()
    }
}
