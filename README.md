# Magic Arbuz Collection

This is the parent (collection) contract for the Magic Arbuz project. It manages minting and indexing of orbital (child) contracts, each of which generates a unique text prediction on demand.

## Features
- **Unlimited deterministic minting**: Mint as many orbitals as you want, each with a unique prediction.
- **No storage of predictions**: All predictions are generated on-the-fly, nothing is pre-stored.
- **Simple parent-child architecture**: Each orbital is linked to this collection.

## Build
```bash
cargo build --target wasm32-unknown-unknown --release
```
The compiled WASM will be in `target/wasm32-unknown-unknown/release/magic_arbuz.wasm`.

## Deploy
```bash
oyl alkane new-contract -c ./target/wasm32-unknown-unknown/release/magic_arbuz.wasm -data 1,0 -p regtest
```

## Minting
Mint a new orbital (child):
```bash
oyl alkane execute -data 2,tx,77 -p regtest
```

## Get Prediction
Get the prediction for a specific orbital by index:
```bash
oyl alkane execute -data 2,tx,1000,0 -p regtest
```