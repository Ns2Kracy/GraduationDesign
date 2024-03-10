pub mod openapi;

use axum::routing::get;

use crate::{ApiResponse, ApiResult};

async fn ping() -> ApiResult<()> {
    Ok(ApiResponse {
        code: 200,
        message: "pong".to_string(),
        data: None,
    })
}

pub fn mount() -> axum::Router {
    axum::Router::new()
        .route("/ping", get(ping))
        .nest("/api", axum::Router::new().merge(openapi::mount()))
}
