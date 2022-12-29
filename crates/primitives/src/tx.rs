use crypto::core_types::{Address};
use serde::{Serialize, Deserialize};


/* 
    - Support transaction decodings for: 
        1. creating an account
        2. transferring native `rust-chain` balances
        
    - Construct general transaction structure

    - TX validation
*/

// struct CreateTx {
//     Address: Box<str>,
//     PublicKey: Box<str>,
//     Message: Box<str>,
// }

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TransferTx {
    from: Address,
    to: Address,
    amount: u32,
}

pub enum TxType {
    Create,
    Transfer,
}

pub struct Transaction {
    pub nonce: u32,
    pub type_: TxType,
}