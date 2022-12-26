use std::time::{SystemTime};

use crate::tx;
use crypto::core_types::{Hash, Address};

// TODO: put timestamp as relevant block field
// TOOD: move to /core/ dir
// TODO: create proper constructor
// TODO: figure optimal hashing library to utilize


pub struct Header {
    pub parent_hash: Hash,
    pub block_hash: Hash,
    pub height: u64,
    pub timestamp: SystemTime,
    pub tx_list: Vec<tx::Transaction>,
    // TxRoot ... Build/import merkle tree 
}

pub struct Block {
    pub header: Header,
}


impl Block {

    // new ... block constructor 
    pub fn new(parent_hash: &Hash, block_hash: &Hash, height: u64) -> Self {

        let block_header: Header = Header {
            block_hash: block_hash.clone(),
            parent_hash: parent_hash.clone(),
            height: height,
            timestamp: SystemTime::now(),
            tx_list: vec![],
        };

        return Self {
            header: block_header,
        };
    }


}