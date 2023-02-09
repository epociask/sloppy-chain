use std::collections::HashMap;
use bytes::Bytes;
use primitives::Address;

#[derive(Clone, Debug)]
pub struct SloppyState {
    lookup: HashMap,
}

impl SloppyState {
    pub fn new() -> SloppyState {
        return Self {
            lookup: HashMap::new(),
        }
    }

    pub fn insert(key: Address, value: Address) {
        match lookup.insert(key, value) {
            Some(prev) => println!("Removed old entry from state {}", prev),
            None(_) => _,
        }
    }

    pub fn get(key: Address, value: Address) -> Address {
        match lookup.get(key, value) {
            Some(val) => val,
            None(_) => primitives::null_address(),
        }
    }

}