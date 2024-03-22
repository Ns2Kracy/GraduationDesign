use axum::Router;

pub fn mount() -> Router {
    Router::new().nest("/alert", Router::new())
}
