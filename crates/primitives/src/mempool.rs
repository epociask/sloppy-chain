/*
    Construct lightweight mem pool represented by priority queue

*/

use crate::tx::TransferTx;

pub struct MemPool {
    pub pool: Vec<TransferTx>
}

impl MemPool {
 
    pub fn new(self) -> MemPool {
        return Self {
            pool: vec![],
        }
    }

    pub fn insert_tx(mut self, transaction: TransferTx) -> Result<u8, &'static str> {
        self.pool.push(transaction);
    }

    pub fn pop_tx(mut self) -> Option<TransferTx> {
        return self.pool.pop();
    }
}