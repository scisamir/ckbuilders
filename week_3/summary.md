# Weekly Progress Report - Week 3

## Overview
This week, I worked through two practical Nervos CKB dApp tutorials:

- Create a Fungible Token
- Create a DOB

The main focus of the week was understanding how CKB represents different kinds of digital assets on-chain. I learned how fungible tokens are created with xUDT and how digital objects can be created and rendered using the Spore Protocol.

## Completed Tutorials

### 1. Create a Fungible Token
- Learned that custom tokens on CKB are called User-Defined Tokens (UDTs), and that xUDT is the minimal extensible standard used to issue them.
- Understood that unlike account-based token standards such as ERC20, tokens on CKB are represented through Cells.
- Learned that issuing a token means creating a Cell whose `type` script is the xUDT script and whose `data` field stores the token amount.
- Understood that the issuer's lock script hash is used as the unique identifier for the token, which becomes the `args` of the xUDT type script.
- Saw how the CCC SDK is used to construct the transaction, add required cell dependencies, complete inputs by capacity, calculate fees, sign, and send the transaction.

### 2. Create a DOB
- Learned that a DOB on CKB can be created using the Spore Protocol, which stores digital objects directly on-chain.
- Understood that a Spore Cell contains structured data such as `content-type` and `content`, allowing files like images to become digital objects on the blockchain.
- Learned that Spore Cells are immutable after creation, which makes the stored digital object permanent in its created form.
- Saw how file content from the browser can be read as an `ArrayBuffer`, converted into a `Uint8Array`, and then passed into the transaction as the digital object's content.
- Learned that the Spore SDK simplifies creation of the digital object Cell and handles transaction building before signing and broadcasting it.

## What I Understood

### Fungible Tokens in the Cell Model
I understood that creating a fungible token on CKB is really about creating a special output Cell. That Cell contains the token amount in its `data` field and uses the xUDT `type` script to define the token's behavior.

### Token Identity and Ownership
I learned that the identity of a token is tied to the issuer's lock script hash. I also understood that ownership is determined by the lock script attached to each token Cell, and transferring tokens means creating a new Cell with the receiver's lock script.

### Digital Objects as On-Chain Assets
I understood that CKB can also store richer digital assets, not just token balances or plain text. With Spore, a Cell can represent a digital object whose actual content, such as an image, is encoded directly on-chain.

### Spore Cell Structure
I learned that a Spore Cell includes both metadata and binary content. The `content-type` tells the application how to interpret the file, while the `content` holds the actual bytes of the object.

### Reading and Rendering a DOB
I understood that after creating a digital object, it can be retrieved by locating the correct live Cell using its transaction hash and output index. The stored data can then be unpacked, converted back into binary form, and rendered in the browser.

## Key Takeaways
- CKB can represent different asset types through Cells, including fungible tokens and digital objects.
- xUDT is used for fungible tokens, while Spore Protocol is used for DOBs.
- A token amount is stored in a Cell's `data` field, and a digital object's content is also stored directly in Cell data.
- Lock scripts define ownership, while type scripts define the rules and structure of the asset.
- Building on CKB requires understanding how transactions create, transform, and expose on-chain assets through Cells.

## Summary
Week 3 helped me understand that Nervos CKB is not limited to simple transfers or balances. I learned how fungible tokens are issued and transferred with xUDT, and how digital objects can be created and displayed on-chain using Spore. This gave me a stronger understanding of how flexible the Cell model is for representing different forms of blockchain state and assets.
