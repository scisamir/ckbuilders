# Weekly Progress Report — CKB Builder Handbook

## Overview
This week, I focused on building a strong foundation in the Nervos CKB ecosystem by completing the introductory modules of the CKB Builder Handbook and beginning hands-on setup for development.

---

## Completed Modules

### 1. Introduction to Nervos CKB
- Gained an understanding of the core architecture of Nervos CKB as a Layer 1 blockchain.
- Learned key concepts such as:
  - Cell model vs account model
  - Capacity (CKBytes) and state storage
  - Scripts as the foundation of programmability
- Understood how CKB differs from other blockchains like Ethereum and Cardano, especially in terms of flexibility and low-level design.

---

### 2. Getting Started on CKB

#### Quick Start (OffCKB Setup)
- Set up a local development environment using **OffCKB**.
- Learned how to:
  - Bootstrap a CKB development project
  - Connect to testnet/devnet environments

---

### 3. CKB Academy (Lessons 1 & 2)
- Strengthened theoretical understanding of:
  - CKB transaction structure (inputs, outputs, cell deps, witnesses)
  - How capacity flows through transactions
  - The role of lock scripts and type scripts
- Built intuition around how state transitions are validated on-chain.

---

### 4. Introduction to Scripts (Smart Contracts on CKB)
- Learned that all smart contract logic in CKB is implemented through **scripts**.
- Understood the difference between:
  - **Lock scripts** (ownership and authorization)
  - **Type scripts** (state validation and business logic)
- Explored how scripts are executed in the CKB VM (RISC-V based).
- Gained initial exposure to how custom logic can be attached to cells for advanced use cases.

---

## Key Takeaways
- CKB provides a **low-level, highly flexible model** for building decentralized systems.
- The **cell model** enables more composable and expressive state management compared to traditional account-based systems.
- Understanding **transaction structure and scripts** is critical before building real applications.
- Development on CKB requires thinking in terms of **state transitions and validation rules**, not just contract functions.

---

## Next Steps
- Dive deeper into transaction construction and signing
- Build simple scripts (lock/type) to understand validation flow
- Explore Lumos / SDK tooling for higher-level development
- Start implementing a small prototype (e.g., basic UTXO transfer or custom script)

---

## Summary
This week was focused on **foundational knowledge and environment setup**, positioning me to start building and experimenting with real applications on Nervos CKB.
