use axum::Router;

pub fn mount() -> Router {
    Router::new().nest("/permission", Router::new())
}
