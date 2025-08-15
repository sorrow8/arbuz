# This Is Magic Arbuz Collection Contract

<p align="center">
  <img src="./arbuz_logo.svg" alt="ARBUZ Logo">
</p>

### For Local Testing:
## Build
```bash
cargo build --target wasm32-unknown-unknown --release
```

## Deploy
```bash
oyl alkane new-contract -c ./target/wasm32-unknown-unknown/release/magic_arbuz.wasm -data 1,0 -p regtest
```

## Trace
```bash
oyl alkane trace -params '{"txid":"b29498ec78efa7b7f36c9010c1c7b72bf3992bb1252fca429b22caf34e942764","vout":3}' -p regtest
```

## Simulate
```bash
oyl alkane simulate -p regtest -target 2:id -inputs opcode,index_if_applicable
```