use axum::Router;

mod alert;
mod analysis;
mod assessment;
mod complaint;
mod enforcement_inspection;
mod feedback;
mod openapi;
mod permission;
mod planting;
mod quality_inspection;
mod report;
mod statistical_query;
mod system;
mod user;

pub fn mount() -> Router {
    Router::new().nest(
        "/api",
        Router::new()
            .merge(openapi::mount())
            .merge(alert::mount())
            .merge(analysis::mount())
            .merge(assessment::mount())
            .merge(complaint::mount())
            .merge(enforcement_inspection::mount())
            .merge(feedback::mount())
            .merge(permission::mount())
            .merge(planting::mount())
            .merge(quality_inspection::mount())
            .merge(report::mount())
            .merge(statistical_query::mount())
            .merge(system::mount())
            .merge(user::mount()),
    )
}
