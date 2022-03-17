use std::time::{SystemTime};

// TODO: put timestamp as relevant block field
// TOOD: move to /core/ dir
// TODO: create proper constructor
// TODO: figure optimal hashing library to utilize


pub struct Header {
    pub parent_hash: u8,
    pub hash: Vec<u8>,
    pub height: i32,
    pub timestamp: SystemTime,
    // TxRoot ... Build/import merkle tree 

}

pub struct Block {
    pub header: Header,
}

