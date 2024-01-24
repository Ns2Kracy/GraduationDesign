use axum::Router;
use utoipa::OpenApi;
use utoipa_redoc::{Redoc, Servable};

#[derive(OpenApi)]
#[openapi(paths(), components(schemas()), tags())]
struct ApiDoc;

pub fn mount() -> Router {
    Router::new().merge(Redoc::with_url("/redoc", ApiDoc::openapi()))
}
