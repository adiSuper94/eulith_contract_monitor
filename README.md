# eulith_contract_monitor

## Setup Local Chain
I am using [foundry](https://github.com/foundry-rs/foundry) to setup the basic local infra. Foundry seems to have pretty good documentation. Almost every setup step below is from the [Foundry Book](https://book.getfoundry.sh/). The section of the doc is meant to serve as a quick and dirty startup guide. For any details, refer the offical documentation.

- Install foundry : `curl -L https://foundry.paradigm.xyz | bash`
- Foundry ships with a tool called `anvil`,  use it to spin up a local chain.

Once contracts are built and `anvil` has spun up, we can depoly the contract on to the chain. 
To deploy via cli: To do this run `forge create ${contract_name} --private-key ${key} --rpc-url ${local_chain_url}`

Note: contract should have already been built using `forge` before deployment.


## Dev Setup
- Clone this repo, `cd` into the `contracts` directory.
- The first time you setup the project you will have to run `forge install` to download/clone dependencies.
- run `forge build`. This will compile the solidity contracts, and dump them under `contracts/out/`. Ensure that `out` has the compiled jsons. Our main programs will read files from this location. 
- `cd` back to the root dir, and run `cargo run`.


### Libs used
[Cargo.toml](./app/Cargo.toml)
