# tulipchain

Tulipchain is a blockchain for exchanging tulips. It's fairly similar to the Bitcoin blockchain, but simpler:

- TODO

Each participating node runs a server which exposes a JSON API from which they can see their version of the chain, make transactions, mine blocks, etc. Because it makes use to [Rocket](https://crates.io/crates/rocket) to power this, it must be run with rust nightly.
