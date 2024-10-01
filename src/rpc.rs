use anyhow::{bail, Result};
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

    pub async fn run(&self) -> Result<()> {
        let app = Router::new().route("/", put(|| async { "Transaction Received" }));

        let listener_result = tokio::net::TcpListener::bind(self.url.clone()).await;
        if let Err(e) = listener_result {
            bail!(e);
        }
        let listener = listener_result.unwrap();

        if let Err(e) = axum::serve(listener, app).await {
            bail!(e);
        }

        Ok(())
    }
}
