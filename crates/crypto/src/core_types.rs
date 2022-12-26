use rand::Rng;
use sha3::{Digest, Sha3_256};
use serde::{Serialize, Deserialize};

const AddressLength: usize = 20;
const HashLength: usize = 32;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Hash {
    // 32 bytes or 256 bits for hash value
    value: [u8;HashLength]
}

impl Hash {
    fn new(self) -> Hash {
        return Self {
            value: [0;HashLength],
        }
    }
    
    // TODO - implement
    // fn set_from_32_bytes(mut self, byte_32_value: u32) {
    // }

    fn value(self) -> [u8;HashLength]  {
        return self.value;
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Address {
    // 20 bytes or 160 bits for address value
    value: [u8;AddressLength]
}  

impl Address {
    fn new(self) -> Address {
        return Self {
            value: [0;AddressLength],
        }
    }

    // TODO - implement
    // fn set_from_32_bytes(mut self, byte_32_value: u32) {
    // }

    fn value(self) -> [u8;AddressLength]  {
        return self.value;
    }
}



pub fn generate_random_number() -> u64 {
    let mut rng = rand::thread_rng();
    let rndnum = rng.gen();

    rndnum
}