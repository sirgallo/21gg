# general

### good to know concepts for developing on Solana


## core concepts

### transactions

A signed data structure containing instructions for the network to perform an operation. Transactions are needed for write operations on-chain, while read operations can be performed without a transaction.

`fields`:
1. Signatures: An array of digital signatures from the transaction's signers.
2. Message: The actual instructions that the transaction is issuing to the network.
  1. Message header: 3 uint8s describing how many accounts will sign the payload, how many won’t, and how many are read-only.
  2. Account addresses: an array of addresses of the accounts that will be used in the transaction.
  3. Recent blockhash: a unique value that identifies a recent block - this ensures the transaction is not too old and is not re-processed.
  4. Instructions: which program to call, which accounts to use, and any additional data needed for the program to execute the instruction.

### instructions

Instructions are the base operational unit on Solana, which contains:
```json
{
  ProgramID: <the program public key>,
  Accounts: <array of accounts the instruction will read/write>,
  Data: <additional information or parameters for the instructions, in byte array form>
}
```

### accounts

Accounts are units of storage on Solana, which can hold generic data up to `10MB`. Progams are stateless since data is stored using accounts. If an account stores executable code, it is marked as executable and can process instructions.

### programs

Programs are stateless and process instructions from both users and other programs.


## dev workflow

For more in depth information, check out [workflow](https://solana.com/docs/intro/dev).

- program development

Develop on chain programs in Rust to interact with. 
Once deployed, are publically available through RPC interactions.

- client development

Develop dApps that communicate with on chain programs.