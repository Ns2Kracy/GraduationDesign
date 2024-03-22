use axum::Router;

pub fn mount() -> Router {
    Router::new().nest("/quality_inspection", Router::new())
}
