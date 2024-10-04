use crate::poh::Poh;
use anyhow::{bail, Result};
use axum::{extract::Json, routing::put, Router};
use spinners::{Spinner, Spinners};

pub struct Node {
    rpc: String,
    poh: Poh,
}

impl Node {
    pub fn new(base_url: &str, port: &str) -> Self {
        Node {
            rpc: format!("{base_url}:{port}"),
            poh: Poh::new(),
        }
    }

    pub async fn run(&self) -> Result<()> {
        let app = Router::new().route("/", put(queue_transaction));

        let listener_result = tokio::net::TcpListener::bind(self.rpc.clone()).await;
        if let Err(e) = listener_result {
            bail!(e);
        }
        let listener = listener_result.unwrap();

        let run_server = async {
            // ⠋ Node is running...
            let sp = Spinner::new(Spinners::Dots, "Node is running...".into());
            axum::serve(listener, app).await
        };

        tokio::select! {
            result = run_server => {
                if let Err(e) = result {
                    bail!(e);
                }
            }

            _ = tokio::signal::ctrl_c() => {
                println!("Received shutdown signal...");
            }
        }

        println!("Cleaning up...");
        println!("Shutting down...");

        Ok(())
    }
}

// JSON format
//
// {
//      message: 13456534
// }
async fn queue_transaction(Json(payload): Json<serde_json::Value>) -> String {
    let timestamp = payload.get("message").unwrap();
    println!("Payload: {}", payload.get("message").unwrap());
    String::from("Transaction Received")
}
