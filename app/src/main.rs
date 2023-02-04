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
    let contract_monitor = monitor_contract(&res);
    let event_monitor = monitor_contract_events(&web3, &res);
    let mutator = mutate_contract(&res, &accounts[0]);
    tokio::select! {
        _val = contract_monitor =>  {println!("contract monitor exit")},
        _val = event_monitor  => {println!("event monitor exit")},
        _val = mutator => {println!("mutator exit")}
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

async fn monitor_contract(contract: &Contract<Http>) -> web3::contract::Result<()> {
    loop {
        let tx = contract.query("number", (), None, web3::contract::Options::default(), None);
        let res: web3::types::U256 = tx.await?;
        let res: u128 = res.low_u128();
        if res > 5 {
            println!("contract.number is :: {res}! Bye!!!");
            break;
        }
        println!("contract.number is :: {res}");
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
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
                println!("Log:: {:?}", log.topics);
            }
            Err(error) => {
                println!("Error occured while processing event : {error:?}");
                break;
            }
        }
    }
    Ok(())
}

async fn mutate_contract(contract: &Contract<Http>, from: &H160) -> web3::contract::Result<()> {
    let mut counter: u8 = 0;
    loop {
        let tx = contract.call("increment", (), *from, web3::contract::Options::default());
        let _res = tx.await?;
        println!("called inc on contract");
        counter += 1;
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
        if counter > 10 {
            break;
        }
    }
    Ok(())
}
