# Rust Chain Primitives
_author:_ @epociask

## Account
Account model will be similar to Ethereum in that:

- Addresses wil be computed using the following logic:
```
    hash:  Byte[32] = KECCAK(ECDSA_PUB_KEY(PRIV_KEY))
    address: Byte[20] = hash << 20
```

## Block
Block will represent a series of state transitions and hold the following data fields:
- `Header` - Block header
    - `Creator` - Validator that produced this block
    - `Previous_Hash` - Link to previous block hash
    - `Transaction_Trie` - Trie of all transactions within block
    - `State_Trie` - Trie of current world state

## Chain

## Transaction

### V0
Starting out, transaction will be encoded into a Big Endian byte array of the following:
```
[0-19, 20-39]
``` 

- `0-19`: The from address or transaction sender
- `20-39`: The to address 

## Mempool
Mempool will represent a list of unfinalized transactions that are used to 