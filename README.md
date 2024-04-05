# NativeX Node


It is usually best to use the standalone version to start a new project.
All bugs, suggestions, and feature requests should be made upstream in the [Substrate](https://github.com/paritytech/substrate/tree/master/bin/node-template) repository.

# Metadata
unit: NATIVEX
address prefix: 42
decimals: 18
chain type: substrate

## Getting Started

Depending on your operating system and Rust version, there might be additional packages required to compile this template.
Check the [installation](https://docs.substrate.io/install/) instructions for your platform for the most common dependencies.
Alternatively, you can use one of the [alternative installation](#alternative-installations) options.

### Build

Use the following command to build the node without launching it:

```sh
make run-release
```
#### Start the first node

```
./target/release/nativex \
  --base-path /tmp/node01 \
  --chain ./customSpecRaw.json \
  --port 30333 \
  --rpc-port 9945 \
  --telemetry-url "wss://telemetry.polkadot.io/submit/ 0" \
  --validator \
  --rpc-methods Unsafe \
  --name NativexNode01 \
  --password-interactive
  ```

 #### Enable other participants to join
 
 ```
./target/release/nativex \
  --base-path /tmp/node02 \
  --chain ./customSpecRaw.json \
  --port 30334 \
  --rpc-port 9946 \
  --telemetry-url "wss://telemetry.polkadot.io/submit/ 0" \
  --validator \
  --rpc-methods Unsafe \
  --name NativexNode02 \
  --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/{NativexNode01Id} \
  --password-interactive

 ``` 


Development chains:

- Maintain state in a `tmp` folder while the node is running.
- Use the **Alice** and **Bob** accounts as default validator authorities.
- Use the **Alice** account as the default `sudo` account.
- Are preconfigured with a genesis state (`/node/service/chain_spec/development.rs`) that includes several prefunded development accounts.


### Connect with Polkadot-JS Apps Front-End

After you start the node template locally, you can interact with it using the hosted version of the [Polkadot/Substrate Portal](https://polkadot.js.org/apps/#/explorer?rpc=ws://localhost:9944) front-end by connecting to the local node endpoint.
A hosted version is also available on [IPFS (redirect) here](https://dotapps.io/) or [IPNS (direct) here](ipns://dotapps.io/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/explorer).
You can also find the source code and instructions for hosting your own instance on the [polkadot-js/apps](https://github.com/polkadot-js/apps) repository.

### Multi-Node Local Testnet

If you want to see the multi-node consensus algorithm in action, see [Simulate a network](https://docs.substrate.io/tutorials/get-started/simulate-network/).

