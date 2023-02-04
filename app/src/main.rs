use core::time;
use serde_json::Value;
use std::{fs, io::Result};
use web3::{
    self,
    contract::Contract,
    ethabi::ParamType,
    futures::StreamExt,
    transports::{self, Http},
    types::{FilterBuilder, Log, H160},
    Web3,
};
const RPC_URL: &str = "http://localhost:8545";
const CONTRACT_MONITOR_SLEEP: u64 = 10;
const EVENT_MONITOR_SLEEP: u64 = 1;
const MUTATOR_SLEEP: u64 = 3;
const EXIT_CONDITION_COUNTER_SIZE: u128 = 5;
const MAX_INCREMENTS_COUNT: u128 = 25;

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
        if res > EXIT_CONDITION_COUNTER_SIZE {
            println!("contract.number is :: {res}! Bye!!!");
            break;
        }
        println!("contract.number is :: {res}");
        tokio::time::sleep(tokio::time::Duration::from_secs(CONTRACT_MONITOR_SLEEP)).await;
    }
    Ok(())
}

async fn monitor_contract_events(web3: &Web3<Http>, contract: &Contract<Http>) -> web3::Result<()> {
    let event_filter = FilterBuilder::default()
        .address(vec![contract.address()])
        .build();

    let filter = web3.eth_filter().create_logs_filter(event_filter).await?;
    let event_stream = filter.stream(time::Duration::from_secs(EVENT_MONITOR_SLEEP));
    web3::futures::pin_mut!(event_stream);
    loop {
        let event = event_stream.next().await.unwrap();
        match event {
            Ok(log) => {
                let count_option = parse_count_from_log(log);
                match count_option {
                    Some(num) => {
                        if num > EXIT_CONDITION_COUNTER_SIZE {
                            println!("EVENT: contract.number is :: {num}! Bye!!!");
                            break;
                        }
                        println!("EVENT: contract.number is :: {num}!");
                    }
                    None => {
                        println!("Error occured while processing Logs");
                    }
                }
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
    let mut counter: u128 = 0;
    loop {
        let tx = contract.call("increment", (), *from, web3::contract::Options::default());
        let _res = tx.await?;
        println!("called inc on contract");
        counter += 1;
        tokio::time::sleep(tokio::time::Duration::from_secs(MUTATOR_SLEEP)).await;
        if counter > MAX_INCREMENTS_COUNT {
            break;
        }
    }
    Ok(())
}

fn parse_count_from_log(log: Log) -> Option<u128> {
    let data = log.data.0.as_slice();
    let token_result = web3::ethabi::decode(&[ParamType::Uint(256)], data);
    match &token_result {
        Ok(tokens) => {
            let uint_number = tokens[0].clone().into_uint()?;
            Some(uint_number.low_u128())
        }
        Err(_err) => None,
    }
}
