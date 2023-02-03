use bindings::counter::Counter;
use ethers::{prelude::*, utils::Anvil};
use std::sync::Arc;

const RPC_URL: &str = "http://localhost:8545";
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provider = Provider::<Http>::try_from(RPC_URL)?;
    let provider = Arc::new(provider);
    let counter = Counter::from(counter);
    Ok(())
}

async fn publish_counter_contract() {
    println!("Hello, world!");
}
