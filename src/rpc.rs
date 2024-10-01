use anyhow::Result;
use axum::{routing::put, Router};

pub struct Rpc {
    url: String,
}

impl Rpc {
    pub fn new(rpc_url: String, port: String) -> Self {
        Rpc {
            url: format!("{rpc_url}:{port}"),
        }
    }

    pub async fn run(&self) {
        let app = Router::new().route("/", put(|| async { "Transaction Received" }));
        let listener = tokio::net::TcpListener::bind(self.url.clone())
            .await
            .unwrap();
        axum::serve(listener, app).await.unwrap();
    }
}
