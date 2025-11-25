mod error;
mod repository;
mod service;
mod state;
mod router;
mod handler;
mod response;
mod model;
mod middleware;
mod extractor;

use tokio::signal;
use log::{error, info};
use sqlx::mysql::MySqlPoolOptions;
use state::AppState;

#[tokio::main]
async fn main() {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(database_url.as_str())
        .await
        .expect("Failed to create database connection pool");

    info!("Database connection pool created");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run database migrations");

    let state = AppState::new(pool);
    let app = router::create_router(state);


    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to address");

    let addr = listener.local_addr().unwrap();
    info!("Server starting on: {}", addr);

    if let Err(e) = axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
    {
        error!("Server error: {}", e);
    }

    info!("Server shutdown complete");
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(unix)]
    let interrupt = async {
        signal::unix::signal(signal::unix::SignalKind::interrupt())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let interrupt = std::future::pending::<()>();

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            info!("received Ctrl+C, shutting down");
        },
        _ = interrupt => {
            info!("received interrupt signal, shutting down");
        },
        _ = terminate => {
            info!("received terminate signal, shutting down");
        },
    }
}
