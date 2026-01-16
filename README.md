# raft-merkle

Cryptographic verification for Raft consensus via Merkle trees.

## How It Works

A `MerkleNode` wrapper around `raft-rs`'s `RawNode` that maintains a Merkle tree of committed Raft log entries, providing:
- Proof that an entry is part of the Raft log
- Commitment to the entire log state
- Verify responses without trusting the node

## Usage

```rust
use raft_merkle::{MerkleNode, MemStorage, Config};
use raft::raft::Config as RaftConfig;


let mut node = MerkleNode::new(&raft_config, storage, &logger)?;

node.commit_entries(&entries);

let root = node.root();

let proof = node.proof(log_index);

```

## API

- commit_entries(entries) - Add committed entries to Merkle tree
- root() - Returns current Merkle root hash (None if empty)
- proof(log_index) - Returns inclusion proof for a log entry


## Example

`main.rs` contains an example 5-node cluster adapted from the raft-rs repository.

```bash
cargo run
```
