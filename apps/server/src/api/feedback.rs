use axum::Router;

pub fn mount() -> Router {
    Router::new().nest("/feedback", Router::new())
}
