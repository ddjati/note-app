mod handler;
mod model;
mod route;
mod schema;
pub mod service;

use dotenv::dotenv;
use model::NoteModel;
use route::create_router;
use tokio::net::TcpListener;

fn _main() {
    let note = NoteModel {
        id: "anu".to_string(),
        title: "judul".to_string(),
        content: "isi".to_string(),
        ..Default::default()
    };
    let json_str = serde_json::to_string(&note).unwrap();
    println!("{}", json_str);
    let note: NoteModel = serde_json::from_str(&json_str).unwrap();
    println!("{:?}", note);
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    println!("🌟 REST API Service 🌟");

    let app = create_router().await;

    println!("✅ Server started successfully at 0.0.0.0:8080");

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to init Ctrl+C handler")
    };

    #[cfg(unix)]
    use tokio::signal::unix;
    let terminate = async {
        unix::signal(unix::SignalKind::terminate())
            .expect("Failed to init signal handler")
            .recv()
            .await
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
    println!("Shutting down gracefully ...")
}
