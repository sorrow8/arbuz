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
oyl alkane trace -params '{"txid":"txid","vout":3}' -p regtest
```

## Minting
```bash
oyl alkane execute -data 2,txid,77 -p regtest
oyl alkane trace -params '{"txid":"txid","vout":3}' -p regtest
```

## Simulate
```bash
oyl alkane simulate -p regtest -target 2:txid -inputs opcode,index
```