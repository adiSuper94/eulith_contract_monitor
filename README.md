# eulith_contract_monitor

## Setup Local Chain
I am using [foundry](https://github.com/foundry-rs/foundry) to setup the basic local infra. Foundry seems to have pretty good documentation. Almost every setup step below is from the [Foundry Book](https://book.getfoundry.sh/). The section of the doc just serves the purpose of showing the high level sequence of steps. For any details, refer the offical documentation. 

- Install foundry : `curl -L https://foundry.paradigm.xyz | bash`
- Build/compile contracts using `forge build`. (I faced an issue with compiling using sol version 0.8.18, so i appended `--use 0.8.17`, you may or may not need to do this.)

Foundry ships with a tool called `anvil` which can be used to spin up a local chain.

Once contracts are built and `anvil` has spun up, we can depoly the contract on to the chain. 
To do this run `forge create ${contract_name} --private-key ${key} --rpc-url ${local_chain_url}`
