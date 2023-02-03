use core::time;
use serde_json::Value;
use std::{fs, io::Result};
use web3::{
    self,
    contract::Contract,
    transports::{self, Http},
    types::H160,
};
const RPC_URL: &str = "http://localhost:8545";

#[tokio::main]
async fn main() -> web3::Result<()> {
    let transport = transports::Http::new(RPC_URL)?;
    let web3 = web3::Web3::new(transport);
    let accounts = web3.eth().accounts().await?;
    let conuter_contract = fetch_counter_contract()?;
    let res = deploy_contract(web3, conuter_contract, accounts[0])
        .await
        .unwrap();
    println!("contract deployment result: {res:?}");
    Ok(())
}

async fn deploy_contract(
    web3: web3::Web3<Http>,
    contract: Value,
    from: H160,
) -> web3::contract::Result<Contract<Http>> {
    let abi = contract["abi"].to_string();
    let bytecode = contract["bytecode"]["object"].to_string();
    let abi_bytes = abi.as_bytes();
    let deployment = Contract::deploy(web3.eth(), abi_bytes)?;
    let xd = deployment
        .poll_interval(time::Duration::from_secs(5))
        .confirmations(0)
        .execute(bytecode, (), from);
    println!("contract deployment in progress");
    Ok(xd.await?)
}

fn fetch_counter_contract() -> Result<Value> {
    let contract_text = fs::read_to_string("./contracts/out/Counter.sol/Counter.json")?;
    let value: Value = serde_json::from_str(contract_text.as_str())?;
    Ok(value)
}
