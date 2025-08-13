# This Is Magic Arbuz Mint Contract

<p align="center">
  <img src="./arbuz.png" alt="ARBUZ Logo">
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
oyl alkane trace -params '{"txid":"txid","vout":3}' -p regtest
```

## Simulate
```bash
oyl alkane simulate -p regtest -target 2:id -inputs opcode,index_if_applicable
```