use axum::{Router, routing::get};
use std::net::SocketAddr;
use tokio::net::TcpListener;

async fn root() -> &'static str {
    "Hello, World!"
}

async fn upload_photo() -> &'static str {
    "Photo uploaded!"
}

async fn delete_photo() -> &'static str {
    "Photo deleted!"
}

#[tokio::main]
async fn main() {
    let app = Router::<()>::new()
        .route("/", get(root))
        .route("/upload", get(upload_photo))
        .route("/delete", get(delete_photo));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Listening on http://{}/", addr);
    axum::serve(listener, app).await.unwrap();
}
