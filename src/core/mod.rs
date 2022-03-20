mod block;
mod chain;
mod crypto; 

use std::time::{SystemTime};

pub fn new_block(parent_hash: u8) -> block::Block {

    let block_header = block::Header{
        parent_hash: parent_hash,
        hash: vec![0,1,2],
        height: 0,
        timestamp: SystemTime::now(),
    };

    block::Block{
        header: block_header,
    }
}

