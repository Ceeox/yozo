use std::{env, sync::Arc};

use anyhow::Result;
use axum::{
    extract::DefaultBodyLimit,
    routing::{get, post},
    Router,
};
use redis::Client;
use tokio::signal;

use crate::{
    files::{load_file, upload_file},
    queue::{get_status, get_status_by_id},
};

pub(crate) struct AppState {
    // TODO: remove me when query is implemented
    #[allow(unused)]
    redis: Client,
}

pub async fn server() -> Result<()> {
    let shared_state = init_shared_state();
    let app = routes(shared_state);
    Ok(axum::Server::bind(
        &env::var("WEBSERVER")
            .expect("missing webserver")
            .parse()
            .unwrap(),
    )
    .serve(app.into_make_service())
    .with_graceful_shutdown(shutdown_signal())
    .await?)
}

fn init_shared_state() -> Arc<AppState> {
    let redis_url = env::var("REDIS_URL").expect("missing redis url");
    let redis = redis::Client::open(redis_url).unwrap();
    let app_state = AppState { redis };
    Arc::new(app_state)
}

fn routes(shared_state: Arc<AppState>) -> Router {
    Router::new()
        .nest("/files", file_routes())
        .nest("/queue", queue_routes(Arc::clone(&shared_state)))
}

fn file_routes() -> Router {
    Router::new()
        .route("/:fileId", get(load_file))
        .route("/upload", post(upload_file))
        .layer(DefaultBodyLimit::max(usize::MAX))
}

fn queue_routes(shared_state: Arc<AppState>) -> Router {
    Router::new()
        .nest("/status", queue_status_routes(Arc::clone(&shared_state)))
        .nest("/presets", queue_presets_routes(Arc::clone(&shared_state)))
}

fn queue_presets_routes(shared_state: Arc<AppState>) -> Router {
    Router::new()
        .route(
            "/",
            get({
                let shared_state = Arc::clone(&shared_state);
                move || get_status(shared_state)
            }),
        )
        .route(
            "/:id",
            get({
                let shared_state = Arc::clone(&shared_state);
                move || get_status(shared_state)
            })
            .post({
                let shared_state = Arc::clone(&shared_state);
                move || get_status(shared_state)
            })
            .delete({
                let shared_state = Arc::clone(&shared_state);
                move || get_status(shared_state)
            }),
        )
}

fn queue_status_routes(shared_state: Arc<AppState>) -> Router {
    Router::new()
        .route(
            "/",
            get({
                let shared_state = Arc::clone(&shared_state);
                move || get_status(shared_state)
            }),
        )
        .route(
            "/:id",
            get({
                let shared_state = Arc::clone(&shared_state);
                move || get_status_by_id(shared_state)
            }),
        )
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

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("signal received, starting graceful shutdown");
}
