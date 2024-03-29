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
use tower_http::{
    cors::CorsLayer,
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
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

    Migrator::up(&db, None).await?;

    let serve_dir = ServeDir::new("dist").not_found_service(ServeFile::new("dist/index.html"));

    let app = Router::new()
        .nest_service("/", serve_dir)
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
        .await?;

    Ok(())
}
