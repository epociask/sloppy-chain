use primitives::mempool;
use primitives::tx::{TransferTx};

use std::sync::Mutex;

pub struct NodeState {
    pub mem_pool: Mutex<mempool::MemPool>
}

impl NodeState {
    pub fn new() -> NodeState {
        return Self {
            mem_pool: Mutex::new(mempool::MemPool::new()),
        }
    }
}