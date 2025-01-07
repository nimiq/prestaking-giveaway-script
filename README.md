# Nimiq Migration Prestaking Rewards Script

This script loads the list of eligible prestakers and their accumulated points and selects a winner for each of the 100 prizes with seeded random number generators.

The RNGs are seeded from a PoS mainnet block height and its block hash.

To run, replace the `BLOCK_NUMBER` and `BLOCK_HASH` constants at the top of `src/main.rs` and run the script with

```bash
cargo run
```
