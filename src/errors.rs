use axum::{
    response::{IntoResponse, Response},
    Json,
};
use hyper::StatusCode;
use serde_json::json;

#[derive(thiserror::Error, Debug)]
#[error("...")]
pub enum Error {
    #[error("{0}")]
    ServerError(#[from] ApiError),
    #[error("{0}")]
    AuthError(#[from] AuthError),
}

impl Error {
    fn error_codes(&self) -> (StatusCode, u16) {
        match self {
            Error::ServerError(_) => (StatusCode::INTERNAL_SERVER_ERROR, 50001),
            Error::AuthError(_) => (StatusCode::UNAUTHORIZED, 40001),
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status, code) = self.error_codes();
        let body = Json(json!({
            "code": code,
            "message": self.to_string(),
        }));
        (status, body).into_response()
    }
}

#[derive(thiserror::Error, Debug)]
#[error("...")]

pub enum ApiError {
    #[error("{0}")]
    InternalServerError(String),
    #[error("{0}")]
    BadRequest(String),
    #[error("{0}")]
    NotFound(String),
    #[error("{0}")]
    Unauthorized(String),
    #[error("{0}")]
    Forbidden(String),
}

#[derive(thiserror::Error, Debug)]
#[error("...")]
pub enum AuthError {
    #[error("Invalid token")]
    InvalidToken,
    #[error("Missing token")]
    MissingToken,
    #[error("Create token error")]
    CreateToken,
    #[error("Expired token")]
    ExpiredToken,
    #[error("Invalid credentials")]
    InvalidCredentials,
}
