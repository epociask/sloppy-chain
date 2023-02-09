
pub type Address = [u8; 30];

pub fn null_address() -> Address {
    let null_address = Default::default();
    return null_address;
}