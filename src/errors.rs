use axum::{
    extract::rejection::{JsonRejection, PathRejection},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

#[derive(Serialize)]
pub struct Body {
    pub status: String,
    pub error: Option<String>,
    pub message: String,
}

impl Body {
    pub fn new<E: ToString>(status: StatusCode, error: E) -> Self {
        Self {
            status: status.as_str().to_owned(),
            error: status
                .canonical_reason()
                .map(|canonical_reason| canonical_reason.to_owned()),
            message: error.to_string(),
        }
    }
}

pub struct Error {
    pub status: StatusCode,
    pub body: Body,
}

impl Error {
    pub fn new<E: ToString>(status: StatusCode, error: E) -> Self {
        Self {
            status,
            body: Body::new(status, error),
        }
    }
}

impl From<JsonRejection> for Error {
    fn from(value: JsonRejection) -> Self {
        Self {
            status: value.status(),
            body: Body::new(value.status(), value.body_text()),
        }
    }
}

impl From<PathRejection> for Error {
    fn from(value: PathRejection) -> Self {
        Self {
            status: value.status(),
            body: Body::new(value.status(), value.body_text()),
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        (self.status, Json(self.body)).into_response()
    }
}
