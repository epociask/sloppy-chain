/*
    Construct lightweight mem pool represented by priority queue

*/

use crate::tx::TransferTx;
use std::sync::Mutex;

pub struct MemPool {
    pub pool: Vec<TransferTx>,
    max_size: usize
}

impl MemPool {

    pub fn new(max_size: usize) -> MemPool {
        return Self {
            pool: Vec::with_capacity(max_size),
            max_size: max_size,
        }
    }

    pub fn insert_tx(&mut self, transaction: TransferTx) -> Result<(), &'static str> {
        if self.pool.len() >= self.max_size {
            Err("MemPool is currently full")
        } else {
            self.pool.push(transaction.clone());
            Ok(())
        }
    }

    pub fn pop_tx(&mut self) -> Option<TransferTx> {
        return self.pool.pop();
    }

    pub fn size(&self) -> u8 {
        return self.pool.len().try_into().unwrap();
    }
}
