/*
    Construct lightweight mem pool represented by priority queue

*/

use crate::tx::TransferTx;
use std::sync::Mutex;

pub struct MemPool {
    pub pool: Vec<TransferTx>
}

impl MemPool {
    // TODO - pass config value here for mempool size
    pub fn new() -> MemPool {
        return Self {
            pool: Vec::with_capacity(100),
        }
    }

    pub fn insert_tx(&mut self, transaction: TransferTx) -> Result<(), &'static str> {
        if self.pool.len() >= 100 {
            return Err("MemPool is currently full");
        }

        self.pool.push(transaction.clone());
        return Ok(());
    }

    pub fn pop_tx(&mut self) -> Option<TransferTx> {
        return self.pool.pop();
    }

    pub fn size(&self) -> u8 {
        return self.pool.len().try_into().unwrap();
    }
}
