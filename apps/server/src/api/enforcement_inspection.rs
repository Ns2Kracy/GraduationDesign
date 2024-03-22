use axum::Router;

pub fn mount() -> Router {
    Router::new().nest("/enforcement_inspection", Router::new())
}
