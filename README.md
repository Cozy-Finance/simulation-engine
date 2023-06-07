# Cozy Simulator

This repo contains a general framework for conducting agent-based simulations on a fresh revm instance and a specific implementation of the simulations for the Cozy protocol. I forked it from [arbiter](https://github.com/primitivefinance/arbiter), but basically stripped away most of the functionality there other than some of the tooling they provided around interacting with revm.

## Repo Structure

- `binder`: Contains code to generate Rust bindings from contract artifacts. Instead of relying on `forge bind`, this is a cli tool that: 1) directly calls into ether-rs `MultiAbigen` and 2) collects some additional metadata (contract names, paths and unlinked bytecode). We need this data to dynamically link library addresses for contracts that have external library dependences inside the simulation. The contract names are also very useful as keys to be used in `sim_data.contract_registry`.

- `lib`: General agent-based simulation framework:
    - `bindings`: Bindings generated by `binder`. We should probably move this out of `lib`.
    - `onchain`: This is from arbiter's initial set-up for pulling/monitoring on-chain data. I have not looked into it.
    - `simulate`: The core simulation logic
        - `contract`: A wrapper around the ethers-rs `BaseContract` that is the main contract object in the simulation.
        - `environment`: Wrapper around revm instance to pull/read account data and execute txs.
        - `agent`: `Agent` trait that has to be implemented by all agents.
        - `block_time_policy`: `BlockTimePolicy` trait that has to be implemented by any block time policy, which defines the rate at which blocks/timestamps are fast-forwarded in the revm instance.
        - `sim_env_data`: `SimEnvData` is meant to store data that agents need to share with each other. For now, all it has is a `sim_data.contract_registry: HashMap<String, SimContract<IsDeployed>>`,  mapping from contract names to the contract instances.
        - `manager`: `SimManager` stores the environment, block time policy, sim data and the agents. It deploys agents, manages them, and is from where we call `manager.run_simulation()`.

- `src/cozy`: Contains the Cozy protocol implementation of the simulation framework. Fairly self-explanatory, but needs clean-up and lots of feature work.

## Generating bindings
```bash
./bind.sh
```
## Running Cozy Sim
```bash
cargo run
```