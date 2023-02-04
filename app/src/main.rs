use core::time;
use serde_json::Value;
use std::{fs, io::Result};
use web3::{
    self,
    contract::Contract,
    futures::StreamExt,
    transports::{self, Http},
    types::{FilterBuilder, H160},
    Web3,
};
const RPC_URL: &str = "http://localhost:8545";

#[tokio::main]
async fn main() -> web3::Result<()> {
    let transport = transports::Http::new(RPC_URL)?;
    let web3 = web3::Web3::new(transport);
    let accounts = web3.eth().accounts().await?;
    let counter_contract = fetch_counter_contract()?;
    let res = deploy_contract(&web3, &counter_contract, &accounts[0])
        .await
        .unwrap();
    println!("contract deployment result: {res:?}");
    let contract_monitor = monitor_contract(&res, &accounts[0]);
    let event_monitor = monitor_contract_events(&web3, &res);
    tokio::select! {
        _val = contract_monitor =>  {println!("a")},
        _val = event_monitor  => {println!("b")},
    };
    Ok(())
}

async fn deploy_contract(
    web3: &Web3<Http>,
    contract: &Value,
    from: &H160,
) -> web3::contract::Result<Contract<Http>> {
    let abi = contract["abi"].to_string();
    let bytecode = contract["bytecode"]["object"].to_string();
    let abi_bytes = abi.as_bytes();
    let deployment = Contract::deploy(web3.eth(), abi_bytes)?;
    let xd = deployment
        .poll_interval(time::Duration::from_secs(5))
        .confirmations(0)
        .execute(bytecode, (), *from);
    println!("contract deployment in progress");
    Ok(xd.await?)
}

fn fetch_counter_contract() -> Result<Value> {
    let contract_text = fs::read_to_string("./contracts/out/Counter.sol/Counter.json")?;
    let value: Value = serde_json::from_str(contract_text.as_str())?;
    Ok(value)
}

async fn monitor_contract(contract: &Contract<Http>, from: &H160) -> web3::contract::Result<()> {
    loop {
        let tx = contract
            .call("number", (), *from, web3::contract::Options::default())
            .await?
            .into();
        if tx > 5 {
            println!("tx is :: {tx}");
            break;
        }
    }
    Ok(())
}

async fn monitor_contract_events(web3: &Web3<Http>, contract: &Contract<Http>) -> web3::Result<()> {
    let event_filter = FilterBuilder::default()
        .address(vec![contract.address()])
        .build();

    let filter = web3.eth_filter().create_logs_filter(event_filter).await?;
    let event_stream = filter.stream(time::Duration::from_secs(5));
    web3::futures::pin_mut!(event_stream);
    loop {
        let event = event_stream.next().await.unwrap();
        match event {
            Ok(log) => {
                println!("Log:: {log:?}");
            }
            Err(error) => {
                println!("Error occured while processing event : {error:?}");
                break;
            }
        }
    }
    Ok(())
}
