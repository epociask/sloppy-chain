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
### V0 
Block will represent a series of state transitions and hold the following data fields:
- `Header` - Block header
    - `Creator` - Consensus operator that produced this block
    - `Previous_Hash` - Link to previous block hash
    - `Transaction_Trie` - Trie of all transactions within block
    - `State_Trie` - Trie of current world state
    - `Number` - Block height number

## Chain

## Transaction

### V0
Starting out, transaction will be represented as a Big Endian byte array of the following:
```
[0-19, 20-39, 40-59, 60-63]
``` 

- `0-19`: The from address or transaction sender
- `20-39`: The `To` address
- `40-59`: The `From` address
- `60-63`: The Sloppy Coin Amount

## Mempool
Mempool will represent a list of unfinalized transactions that are used to 

## Resource(s)
- [Ethereum Block Structure](https://medium.com/@eiki1212/ethereum-block-structure-explained-1893bb226bd6#:~:text=Ethereum%20has%20two%20types%20of,transaction%20that%20an%20account%20creates.)