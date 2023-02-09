use primitives::mempool;

use std::sync::Mutex;

mod sloppy;

use crate::sloppy::SloppyState;

pub struct NodeState {
    pub mem_pool: Mutex<mempool::MemPool>,
    pub storage: SloppyState,
    // TODO - current block
}

impl NodeState {
    pub fn new(mempool_size: usize) -> NodeState {
        return Self {
            mem_pool: Mutex::new(mempool::MemPool::new(mempool_size)),
            storage: SloppyState::new(),
        }
    }
}