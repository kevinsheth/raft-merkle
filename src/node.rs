use raft::LightReady;
use raft::RawNode;
use raft::Ready;
use raft::Result;
use raft::eraftpb::{ConfState, Message};
use raft_proto::ConfChangeI;

use raft::Raft;
use raft::storage::Storage;
use slog::Logger;

pub struct MerkleNode<T: Storage> {
    inner: Option<RawNode<T>>,
    pub raft: Raft<T>,
}

impl<T: Storage> MerkleNode<T> {
    pub fn new(config: &raft::Config, store: T, logger: &Logger) -> Result<Self> {
        todo!()
    }

    pub fn tick(&mut self) {
        if let Some(ref mut inner) = self.inner {
            inner.tick();
        }
    }

    pub fn step(&mut self, m: Message) {
        todo!()
    }

    pub fn has_ready(&self) -> bool {
        todo!()
    }

    pub fn ready(&self) -> Ready {
        todo!()
    }

    pub fn advance(&mut self, rd: Ready) -> LightReady {
        todo!()
    }

    pub fn advance_apply(&self) {
        todo!()
    }

    pub fn propose(&mut self, context: Vec<u8>, data: Vec<u8>) -> Result<()> {
        todo!()
    }

    pub fn propose_conf_change(&mut self, context: Vec<u8>, cc: impl ConfChangeI) -> Result<()> {
        todo!()
    }

    pub fn apply_conf_change(&mut self, cc: &impl ConfChangeI) -> Result<ConfState> {
        todo!()
    }
}
