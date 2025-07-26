## Build
```bash
cargo build --target wasm32-unknown-unknown --release
```

## Deploy
```bash
oyl alkane new-contract -c ./target/wasm32-unknown-unknown/release/magic_arbuz.wasm -data 1,0 -p regtest
```

## Gen block
```bash
oyl regtest genBlocks -c -p regtest
```

## Trace
```bash
oyl alkane trace -params '{"txid":"7e9a8e16cba3ff2ea986afb9422224f6b882f6f253796f827f0c56d6e592cbb7","vout":3}' -p regtest
```

## Minting
```bash
oyl alkane execute -data 2,23,77 -p regtest
oyl alkane trace -params '{"txid":"txid","vout":3}' -p regtest
```

## Simulate
```bash
oyl alkane simulate -p regtest -target 2:txid -inputs opcode,index
```