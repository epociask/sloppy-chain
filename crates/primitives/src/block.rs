use std::time::{SystemTime};

use crate::tx;

// TODO: put timestamp as relevant block field
// TOOD: move to /core/ dir
// TODO: create proper constructor
// TODO: figure optimal hashing library to utilize


pub struct Header {
    pub parent_hash: u8,
    pub hash: Vec<u8>,
    pub height: i32,
    pub timestamp: SystemTime,
    pub tx_list: Vec<tx::Transaction>,
    // TxRoot ... Build/import merkle tree 

}

pub struct Block {
    pub header: Header,
}


impl Block {
    pub fn new(parent_hash: u8) -> Self {

        let block_header: Header = Header {
            parent_hash: parent_hash,
            hash: vec![0,1,2],
            height: 0,
            timestamp: SystemTime::now(),
            tx_list: vec![],
        };

        return Self {
            header: block_header,
        };
    }
}