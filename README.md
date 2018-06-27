# cryptocurrency

Simple example how to write a service with [Exonum](https://github.com/exonum/exonum).

## How to run

To compile, run 

```
cargo build
```

To start one node, run:

```
cargo run -- generate-testnet 1 --output-dir .
cargo run -- run -c validators/0.toml -d db0 --public-api-address 127.0.0.1:3000
```
