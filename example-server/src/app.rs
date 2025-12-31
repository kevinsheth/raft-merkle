use openraft::Raft;
use openraft_memstore::MemStoreStateMachine;
use raft_merkle::types::NodeId;

pub struct App {
    pub id: NodeId,
    pub addr: String,
    pub raft: Raft<raft_merkle::types::TypeConfig>,
    // TODO: Replace with MerkleStateMachine after getting cluster working
    pub state_machine_store: MemStoreStateMachine,
}
