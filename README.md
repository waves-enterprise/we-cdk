# we-cdk

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

## Hello, World! ‒ The Flipper

The `Flipper` contract is a simple contract containing only a single `bool` value.

It provides method flip its value from `true` to `false` (and vice versa).

Below you can see the code using CDK.

```rust
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
