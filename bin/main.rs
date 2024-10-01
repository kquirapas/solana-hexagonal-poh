use solana_hexagonal_poh::prelude::*;

#[tokio::main]
async fn main() {
    let rpc = Rpc::new(String::from("0.0.0.0"), String::from("3000"));

    rpc.run().await;
}
