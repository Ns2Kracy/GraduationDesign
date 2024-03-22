use axum::Router;

pub fn mount() -> Router {
    Router::new().nest("/report", Router::new())
}
