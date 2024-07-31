# we-cdk

we-cdk [![crates.io](https://img.shields.io/crates/v/we-cdk.svg)](https://crates.io/crates/we-cdk)
cargo-we [![crates.io](https://img.shields.io/crates/v/cargo-we.svg)](https://crates.io/crates/cargo-we)

Toolkit for development WASM smart-contracts on Waves Enterprise ecosystem.

## Usage

A prerequisite for compiling smart contracts is to have Rust and Cargo installed. Here's [an installation guide](https://doc.rust-lang.org/cargo/getting-started/installation.html).

To work with the contract it is necessary to install `cargo-we`. It's a CLI tool which helps set up and manage WASM smart contracts.

```
cargo install cargo-we --force
```

Use the `--force` to ensure you are updated to the most recent `cargo-we` version.

And there is also an option to install the CLI tool from this project folder.

```
cargo install --path ./crates/cargo-we/ --force
```

In order to initialize a new project you can use:

```
cargo we new flipper
```

This will create a folder `flipper` in your work directory.
The folder contains a scaffold `Cargo.toml` and a `lib.rs`, which both contain the necessary building blocks for using WASM smart-contract.

The `lib.rs` contains our hello world contract ‒ the `Flipper`, which we explain in the next section.

In order to build the contract just execute this command in the `flipper` folder:
```
cargo we build
```

As a result you'll get a `target/we/flipper.wasm` file and a `flipper.json` file in the `target/we` folder of your contract.

## Create & Update contract

With `cargo-we` can also send `CreateContract` and `UpdateContract` transactions directly to a network node.

To do this, create a file with a description of access to the node and the transaction to be sent.

To send a transaction, all you need to do is execute the command:
```
cargo we tx --send <path_json>
```

Example JSON file:

```json
{
    "nodeUrl": "http://localhost:6862",
    "apiKey": "we",
    "transaction": {
        "type" : 103,
        "version" : 7,
        "sender" : "3NA9hBGoVPfJVybremiFgWN8REi9oiDydEF",
        "password": "",
        "contractName": "flipper",
        "params": [
            {
                "type": "boolean",
                "key": "init_value",
                "value": false
            }
        ],
        "isConfidential": false,
        "fee" : 100000000,
        "payments" : [ ],
        "feeAssetId" : null,
        "validationPolicy" : {
            "type": "any"
        },
        "groupOwners" : [ ],
        "groupParticipants" : [ ]
    }
}
```

## Hello, World! ‒ The Flipper

The `Flipper` contract is a simple contract containing only a single `bool` value.

It provides method flip its value from `true` to `false` (and vice versa).

Below you can see the code using CDK.

```rust
#![no_std]
#![no_main]
use we_cdk::*;

// Declaring a function available for calling.
// `#[action]` keyword is used for this purpose.
// _constructor mandatory method that is called during CreateContract Transaction.
#[action]
fn _constructor(init_value: Boolean) {
    // Write the obtained value as an argument of the function into contract state.
    set_storage!(boolean :: "value" => init_value);
}

#[action]
fn flip() {
    // Read the value from the contract state.
    let value: Boolean = get_storage!(boolean :: "value");
    // Write the inverted value to the contract state.
    set_storage!(boolean :: "value" => !value);
}
```
