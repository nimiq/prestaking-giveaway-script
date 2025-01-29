# Nimiq Pre-staking Giveaway Script

The pre-staking giveway prizes are calculated in three rounds. Each round's random-number-generators (RNGs) are seeded by an election block's number and hash.

Each address can only win once, and winning addresses are removed from the list of eligible addresses for the following rounds.

This script loads the list of eligible prestaker addresses and their accumulated points for each round and selects a winner for each of the round's prizes.

The list of prizes is shuffled with an RNG seeded by the block number, the winners are selected with an RNG seeded by the block hash, using the same `DiscreteDistribution` mechanism that is used to distribute validator slots on the PoS chain.

To run, pass the round number (1-3), the block number and the block hash (in this order) as arguments to the script, for example:

```bash
# Run the script for round 1, with block number 7557700 and hash 6bede69541d2cf748f023866d533deaf3819212e40e2af51012c6e1e825f6d95
cargo run -- 1 7557700 6bede69541d2cf748f023866d533deaf3819212e40e2af51012c6e1e825f6d95
```

Following the production of each round's election block, this repository will be updated with the round's winners and the list of prestakers eligible for the next round.

## Round 1

_Coming February 3, 2025_

## Round 2

_Coming February 10, 2025_

## Round 3

_Coming February 17, 2025_
