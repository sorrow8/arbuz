# This Magic Arbuz Mint Contract

## Build
```bash
cargo build --target wasm32-unknown-unknown --release
```

## Deploy
```bash
oyl alkane new-contract -c ./target/wasm32-unknown-unknown/release/magic_arbuz.wasm -data 1,0 -p network
```

## Trace
```bash
oyl alkane trace -params '{"txid":"tx_id","vout":3}' -p network
```

## Mint
```bash
oyl alkane execute -data 2,id,77 -p network
```

## Simulate
```bash
oyl alkane simulate -p network -target 2:id -inputs opcode,index_if_applicable
```