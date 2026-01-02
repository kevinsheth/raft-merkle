use raft::LightReady;
use raft::RawNode;
use raft::Ready;
use raft::Result;
use raft::StateRole;
use raft::eraftpb::{ConfState, Message};
use raft_proto::ConfChangeI;

use raft::storage::Storage;
use slog::Logger;

pub struct MerkleNode<T: Storage> {
    inner: RawNode<T>,
}

impl<T: Storage> MerkleNode<T>
where
    T: Clone,
{
    pub fn new(config: &raft::Config, store: T, logger: &Logger) -> Result<Self> {
        let inner = RawNode::new(config, store, logger)?;

        Ok(Self { inner })
    }

    pub fn is_leader(&self) -> bool {
        self.inner.raft.state == StateRole::Leader
    }

    pub fn store(&self) -> T {
        self.inner.raft.raft_log.store.clone()
    }

    pub fn last_index(&self) -> u64 {
        self.inner.raft.raft_log.last_index()
    }

    pub fn tick(&mut self) {
        self.inner.tick();
    }

    pub fn step(&mut self, m: Message) {
        let _ = self.inner.step(m);
    }

    pub fn has_ready(&self) -> bool {
        self.inner.has_ready()
    }

    pub fn ready(&mut self) -> Ready {
        self.inner.ready()
    }

    pub fn advance(&mut self, rd: Ready) -> LightReady {
        self.inner.advance(rd)
    }

    pub fn advance_apply(&mut self) {
        self.inner.advance_apply();
    }

    pub fn propose(&mut self, context: Vec<u8>, data: Vec<u8>) -> Result<()> {
        self.inner.propose(context, data)
    }

    pub fn propose_conf_change(&mut self, context: Vec<u8>, cc: impl ConfChangeI) -> Result<()> {
        self.inner.propose_conf_change(context, cc)
    }

    pub fn apply_conf_change(&mut self, cc: &impl ConfChangeI) -> Result<ConfState> {
        self.inner.apply_conf_change(cc)
    }
}
