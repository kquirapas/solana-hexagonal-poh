use crate::poh::Poh;
use anyhow::{bail, Result};
use axum::{
    extract::{Json, State},
    routing::put,
    Router,
};
use spinners::{Spinner, Spinners};

pub struct Node {
    rpc: String,
    poh: Poh,
}

impl Node {
    pub fn new(base_url: &str, port: &str) -> Self {
        Node {
            rpc: format!("{base_url}:{port}"),
            // TODO: Add parameter for buffer size
            poh: Poh::new(None),
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        let app = Router::new().route("/", put(queue_transaction));
        // .with_state(shared_poh);

        let listener_result = tokio::net::TcpListener::bind(self.rpc.clone()).await;
        if let Err(e) = listener_result {
            bail!(e);
        }
        let listener = listener_result.unwrap();

        let run_server = async {
            // ⠋ Node is running...
            let _sp = Spinner::new(Spinners::Dots, "Node is running...".into());
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
    let message = payload.get("message").unwrap().as_str().unwrap();
    let timestamp: u64 = payload.get("timestamp").unwrap().as_u64().unwrap();

    println!("message: {}", message);
    println!("timestamp: {}", timestamp);

    String::from("transaction queued")
}
