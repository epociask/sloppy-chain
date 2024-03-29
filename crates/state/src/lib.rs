use primitives::mempool;

use std::sync::Mutex;

pub struct NodeState {
    pub mem_pool: Mutex<mempool::MemPool>
}

impl NodeState {
    pub fn new(mempool_size: usize) -> NodeState {
        return Self {
            mem_pool: Mutex::new(mempool::MemPool::new(mempool_size)),
        }
    }
}