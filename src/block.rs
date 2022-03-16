use std::time::{SystemTime};

// TODO: put timestamp as relevant block field
// TOOD: move to /core/ dir
// TODO: create proper constructor
// TODO: figure optimal hashing library to utilize

pub struct Header {
    parent_hash: u8,
    hash: u8,
    pub height: i32,
    // TimeStamp ... Figure out how to load datetime library
    // TxRoot ... Build/import merkle tree 

}

pub struct Block {
    pub header: Header,
}

pub fn new_block(parent_hash: u8) -> Block {
    let block_header = Header{
        parent_hash: parent_hash,
        hash: 0,
        height: 0,
    };

    Block{
        header: block_header,
    }
}