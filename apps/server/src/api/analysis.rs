use axum::Router;

pub fn mount() -> Router {
    Router::new().nest("/analysis", Router::new())
}
