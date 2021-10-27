# JS vs RUST getProgramAccounts benchmark 

> Repro project for https://github.com/solana-labs/solana/issues/21022

This repo project demonstrates that getProgramAccounts takes x10 longer on Rust vs JS. Both programs do the same thing, get obligation program accounts from the SolendProtocol.

To look at the source code of each program go to `rust/main.rs` and `js/index.js`.

## How to run it
The `run_benchmark.sh` script builds and runs both executables:

```sh
chmod +x run_benchmark.sh
./run_benchmark.sh
```

Example output:
```log
=======================================
Running Rust benchmark
Retrieved 129164 accounts in 109953ms.
=======================================
Running JS benchmark
Retrieved 129164 accounts in 18550ms.
=======================================
```
