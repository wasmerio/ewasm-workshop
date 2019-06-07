# Rust SmartContract runtime

This repo showcases how to use the [wasmer-runtime](https://crates.io/crates/wasmer-runtime/) from Rust, based on the blogpost: https://medium.com/wasmer/executing-webassembly-in-your-rust-application-d5cd32e8ce46

See [`src/main.rs`](./src/main.rs) for the example implementation.

The `smartcontract` directory contains an example start contract app to run in the embedder app.

## Requirements
- Rust `nightly` - install using `rustup install nightly`

- Rust target `wasm32-unknown-unknown` - install using `rustup target add wasm32-unknown-unknown`

## Running

```bash
# This will compile the smart contract to a wasm and run it
make run
```
