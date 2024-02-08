use axum::{Extension, Router};
use gd_server::{
    api,
    config::CONFIG,
    context,
    utils::{graceful_shutdown::shutdown_signal, logger::init_tracing},
};
use migration::{Migrator, MigratorTrait};
use sea_orm::Database;
use std::sync::Arc;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

#[tokio::main]
async fn main() {
    init_tracing().await;

    // connect to postgresql
    let db = Database::connect(&CONFIG.postgresql.url())
        .await
        .map_err(|e| {
            tracing::error!("Failed to connect to postgresql: {}", e);
            e
        })
        .unwrap();
    tracing::info!("Connected to postgresql");

    Migrator::up(&db, None).await.unwrap();

    let app = Router::new()
        .merge(api::mount())
        .layer(Extension(Arc::new(context::Context { db })))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind(CONFIG.server.url())
        .await
        .unwrap();

    tracing::info!("Running on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}
