use axum::response::{Html, IntoResponse};
use axum::Router;
use axum::routing::get;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route(
        "/", get(handler_hello)
    );
    let addr = "127.0.0.1:3306";
    println!("->> LISTENING ON http://{addr}\n");
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, routes_hello).await.unwrap();
}

async fn handler_hello() -> impl IntoResponse {
    println!("->> {:<12} - handler_hello", "HANDLER");
    Html("<h1>Hello, World!</h1>")
}