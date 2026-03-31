# Weekly Progress Report - Week 2

## Overview
This week, I worked through two practical Nervos CKB dApp tutorials:

- Transfer CKB
- Store Data on Cell

The main focus of the week was understanding how transactions work in the Cell model and how CKB can be used not only to transfer value, but also to store application data directly on-chain.

## Completed Tutorials

### 1. Transfer CKB
- Learned how CKB transfers are performed by consuming existing live Cells and creating new output Cells.
- Understood that ownership is enforced through lock scripts, which ensure that only the correct private key can authorize spending.
- Saw how the CCC SDK helps build a transaction, fill in missing inputs, calculate transaction fees, sign the transaction, and send it to the network.
- Understood that a transfer on CKB is not just "sending balance" like in an account-based model, but creating a new transaction that reshapes Cells.

### 2. Store Data on Cell
- Learned that a Cell can store arbitrary data in its `data` field.
- Understood that text data must first be encoded into hex before it can be stored on-chain.
- Learned how stored data can be read back by retrieving the correct live Cell and decoding the hex value into readable text.
- Saw that a specific Cell can be identified using its transaction hash and output index (`OutPoint`).

## What I Understood

### Cell Model in Practice
This week made the Cell model more concrete for me. I understood that Cells are the foundation of state on CKB. They can hold CKB capacity, scripts, and arbitrary data, which makes them more flexible than simple account balances.

### Transactions as State Changes
I understood that every action on CKB, whether transferring tokens or storing a message, happens through transaction construction. A transaction consumes old Cells and creates new ones, which means application state changes are expressed as Cell transformations.

### Capacity and Data Storage
I learned that capacity is not only about value, but also about the storage space a Cell occupies on-chain. This helped me understand why storing data in a Cell is tied to how much CKB the Cell must contain.

### Reading and Writing On-Chain Data
I understood that writing data on-chain involves encoding it properly and placing it in a Cell's `data` field. Reading it back requires locating the live Cell and decoding the raw stored value into something meaningful for the application.

## Key Takeaways
- CKB transfers and on-chain data storage both use the same Cell-based transaction model.
- Lock scripts control who can unlock and spend Cells.
- The `data` field of a Cell makes it possible to store application-specific information directly on-chain.
- Building on CKB requires thinking in terms of Cells, capacity, scripts, and transaction flow rather than account balances.

## Summary
Week 2 helped me move from theory into hands-on understanding. I now better understand how Nervos CKB handles both value transfer and data storage through Cells, and how SDK tools like CCC make it easier to build, complete, sign, and broadcast these transactions in a dApp workflow.
