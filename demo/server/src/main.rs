use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use anyhow::Context;
use axum::Router;
use axum_server::Handle;
use tokio::{signal, sync::Notify, task::JoinHandle};
use tower_http::services::ServeDir;

fn build_router(project_root: &Path) -> Router {
    let dist_dir = project_root.join("dist/csr");

    Router::new().nest_service(
        "/csr",
        ServeDir::new(dist_dir).append_index_html_on_directories(true),
    )
}

fn project_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent() // demo/
        .expect("Expected to find parent directory")
        .parent() // frontary-leptos/
        .expect("Expected to find parent directory")
        .to_path_buf()
}

fn serve(addr: &str, project_root: PathBuf) -> (JoinHandle<anyhow::Result<()>>, Arc<Notify>) {
    let addr = addr.to_string();
    let shutdown_notify = Arc::new(Notify::new());
    let shutdown_notify_clone = shutdown_notify.clone();
    let handle = Handle::new();
    let handle_clone = handle.clone();

    let task = tokio::spawn(async move {
        let app = build_router(&project_root);

        let shutdown_task = tokio::spawn(async move {
            shutdown_notify_clone.notified().await;
            handle_clone.graceful_shutdown(Some(Duration::from_secs(5)));
        });

        axum_server::bind(addr.parse().unwrap())
            .handle(handle)
            .serve(app.into_make_service())
            .await
            .context("Failed to run Axum server")?;

        let _ = shutdown_task.await;

        Ok(())
    });

    (task, shutdown_notify)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let project_root = project_root();
    let addr = "127.0.0.1:8446";

    println!("Starting Axum server at http://{addr}");

    let (server_handle, shutdown_notify) = serve(addr, project_root);

    println!("Serving http://{addr}/csr/ to access the CSR demo");

    tokio::select! {
        _ = signal::ctrl_c() => {
            println!("Ctrl+C signal received, shutting down server...");
            shutdown_notify.notify_waiters();

            match server_handle.await {
                Ok(Ok(())) => println!("Server shut down gracefully."),
                Ok(Err(e)) => eprintln!("Server error: {e:?}"),
                Err(e) => eprintln!("Server join error: {e:?}"),
            }
        }
    }

    Ok(())
}
