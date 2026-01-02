# raft-merkle

A `MerkleNode` wrapper around [raft-rs](https://github.com/tikv/raft-rs)'s `RawNode`.

## Status

Work in progress. Currently `MerkleNode` is a passthrough to `RawNode`.

## Roadmap

- [ ] Merkle tree state representation
- [ ] State verification via Merkle proofs
- [ ] Efficient state sync using tree diffs

## Usage

```rust
use raft_merkle::node::MerkleNode;
use raft::storage::MemStorage;

let node = MerkleNode::new(&config, storage, &logger)?;
```

## Example

`main.rs` contains an example 5-node cluster adapted from the raft-rs repository.

```bash
cargo run
```
