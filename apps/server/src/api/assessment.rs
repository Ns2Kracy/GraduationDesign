use axum::Router;

pub fn mount() -> Router {
    Router::new().nest("/assessment", Router::new())
}
