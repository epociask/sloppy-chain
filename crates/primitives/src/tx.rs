
/* 
    - Support transaction decodings for: 
        1. creating an account
        2. transferring native `rust-chain` balances
        
    - Construct general transaction structure

    - TX validation
*/

struct CreateTx {
    Address: Box<str>,
    PublicKey: Box<str>,
    Message: Box<str>,
}

struct TransferTx {
    To: Box<str>,
    Amount: u32,
}

pub enum TxType {
    Create,
    Transfer,
}

pub struct Transaction {
    pub Nonce: u32,
    pub Type: TxType,
}