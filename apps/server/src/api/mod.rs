pub mod openapi;

pub fn mount() -> axum::Router {
    axum::Router::new().nest("/api", axum::Router::new().merge(openapi::mount()))
}
