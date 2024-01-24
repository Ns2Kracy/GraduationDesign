use axum::{
    response::{IntoResponse, Response},
    Json,
};
use errors::Error;
use hyper::StatusCode;
use serde::{Deserialize, Serialize};

pub mod api;
pub mod common;
pub mod config;
pub mod context;
pub mod errors;
pub mod utils;

pub type ApiResult<T> = std::result::Result<ApiResponse<T>, Error>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub code: u16,
    pub message: String,
    pub data: Option<T>,
}

impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        let body = Json(self);

        (StatusCode::OK, body).into_response()
    }
}
