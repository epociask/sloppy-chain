use rand::Rng;
//use sha3::{Digest, Sha3_256};

const ADDRESS_LENGTH: usize = 20;
const HASH_LENGTH: usize = 32;

// 32 bytes or 256 bits for hash value
pub type Hash = [u8 ; HASH_LENGTH];

// 20 bytes or 160 bits for address value
pub type Address = [u8 ; ADDRESS_LENGTH];

pub fn generate_random_number() -> u64 {
    let mut rng = rand::thread_rng();
    let rndnum = rng.gen();

    rndnum
}