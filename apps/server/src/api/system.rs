use axum::Router;

pub fn mount() -> Router {
    Router::new().nest("/system", Router::new())
}
