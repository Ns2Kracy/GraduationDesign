use axum::Router;

pub fn mount() -> Router {
    Router::new().nest("/statistical_query", Router::new())
}
