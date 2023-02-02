# eulith_contract_monitor

## Setup Local Chain
I am using [foundry](https://github.com/foundry-rs/foundry) to setup the basic local infra. Foundry seems to have pretty good documentation. Almost every setup step below is from the [Foundry Book](https://book.getfoundry.sh/). The list below just serves the purpose of showing the high level sequence of steps. For any details, offical documentation. 

- Install foundry : `curl -L https://foundry.paradigm.xyz | bash`
- Init foundry project with basic contract : `forge init eulith_foundry`
- There should be a basic "Counter" contract in under `eulith_foundry/src`
- Build/compile contacts using `forge build`. (I faced an issue with compiling using sol version 0.8.18, so i appended `--use 0.8.17`, you may or may not need to do this.)

Foundry ships with a tool called `anvil` which can be used to spin up a local chain.

Once conrtacts are built and `anvil` is spun up, we can depoly the contact on to the chain. To do this run `forge create ${contract_name} --private-key ${key} --rpc-url ${url_on_which-anvil_spun_up_a_chain}`
