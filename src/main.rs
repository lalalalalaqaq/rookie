use axum::{
    routing::get,
    Router,
};
use std::net::SocketAddr;


#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler)); 

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    // run it with hyper on localhost:3000
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
// 处理器
async fn handler() -> &'static str {
    "Hello, World!"
}
