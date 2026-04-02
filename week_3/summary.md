# Weekly Progress Report - Week 3

## Overview
This week, I worked through the Nervos CKB tutorial on creating a fungible token using the xUDT standard.

The main focus of the week was understanding how custom tokens are represented in the CKB Cell model, how they are issued, how token holders can be queried, and how tokens are transferred between users.

## Completed Tutorial

### 1. Create a Fungible Token
- Learned that custom tokens on CKB are called User-Defined Tokens (UDTs), and that xUDT is the minimal extensible standard used to issue them.
- Understood that unlike account-based token standards such as ERC20, tokens on CKB are represented through Cells.
- Learned that issuing a token means creating a Cell whose `type` script is the xUDT script and whose `data` field stores the token amount.
- Understood that the issuer's lock script hash is used as the unique identifier for the token, which becomes the `args` of the xUDT type script.
- Saw how the CCC SDK is used to construct the transaction, add required cell dependencies, complete inputs by capacity, calculate fees, sign, and send the transaction.

## What I Understood

### Token Creation in the Cell Model
I understood that creating a fungible token on CKB is really about creating a special output Cell. That Cell contains the token amount in its `data` field and uses the xUDT `type` script to define the token's behavior.

### Token Identity
I learned that the identity of a token is tied to the issuer's lock script hash. This means each token type is uniquely identified by the issuer, not by a contract address in the account-model sense.

### Querying Token Holders
I understood that to find issued tokens and their holders, you query live Cells by the xUDT type script. Once those Cells are found, the lock scripts attached to them show who currently controls those token balances.

### Token Transfer Logic
I learned that transferring a token means creating a new token Cell for the receiver with the same xUDT type script but a different lock script. The receiver becomes the new holder because they can unlock that Cell.

### Change and Balance Handling
I also understood that if the sender provides more token amount than is being sent, the remaining balance must be returned in a change Cell. This is similar to how CKB capacity change is handled in normal transactions.

## Key Takeaways
- xUDT is the standard used to create fungible tokens on CKB.
- A token amount is stored in the `data` field of a Cell, not in an account balance.
- The issuer's lock script hash acts as the unique token ID through the xUDT script arguments.
- Token holders can be discovered by querying live Cells with the matching xUDT type script.
- Token transfers happen by creating new Cells with the correct lock and type scripts, while also handling token change and transaction fees.

## Summary
Week 3 helped me understand how fungible tokens are built on Nervos CKB using the Cell model. I now have a clearer idea of how token issuance, token identity, querying holders, and token transfer all work through Cells, scripts, and transaction construction rather than through the account-based token logic used on many other blockchains.
