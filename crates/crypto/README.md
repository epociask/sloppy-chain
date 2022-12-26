# Crypto
Rust Chain will use the following cryptography:
- `Merkle Patricia Trie (MPT)` for representing:
    - Transactions in a block
    - Chain world state

- `ECDSA` for:
    - Generating public and private 
    - Recovering a public key from a signed message
    - Signing a message

-  `SHA3` for:
    - Generating addresses
    - Computing block header hashes

- `RLP` for:
    - Encoding arbitrary transaction data
    - Decoding transaction content