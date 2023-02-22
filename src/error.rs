use axum::response::{IntoResponse, Response};
use hyper::StatusCode;
use thiserror::Error;

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Mime type is not supportet")]
    MimeNotSupported,
    #[error("Not found")]
    NotFound,
    #[error("Missing file id")]
    MissingFileId,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            Error::MimeNotSupported => (StatusCode::BAD_REQUEST, self.to_string()),
            Error::NotFound => (StatusCode::BAD_REQUEST, self.to_string()),
            Error::MissingFileId => (StatusCode::BAD_REQUEST, self.to_string()),
        };

        (status, error_message).into_response()
    }
}
