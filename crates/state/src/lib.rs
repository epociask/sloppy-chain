use primitives::{MemPool};
use primitives::tx::{TransferTx};

pub struct NodeState {
    mem_pool: MemPool
}

impl NodeState {
    fn new(self) -> NodeState {
        return Self {
            MemPool::new(),
        }
    }

    fn process_tx(mut self, tx: TransferTx)  Result<u8, &'static str> {
        return self.mem_pool.insert_tx(tx);
    }

}