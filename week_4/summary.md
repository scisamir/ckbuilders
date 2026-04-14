# Weekly Progress Report - Week 4

## Overview
This week, I completed the Nervos CKB tutorial:

- Build a Simple Lock

The main focus of the week was understanding how to build a custom Lock Script and connect it to a frontend dApp workflow. I learned how `script_args` and witnesses work together to enforce unlock conditions on CKB.

## Completed Tutorial

### 1. Build a Simple Lock
- Learned how to build a full-stack CKB dApp using a custom lock called `hash_lock`.
- Understood that the lock stores an expected hash in `script_args`, and unlocking requires providing the matching preimage in the witness.
- Learned the end-to-end workflow: start Devnet with OffCKB, compile and deploy the script, run the frontend, deposit CKB, and test unlock/transfer.
- Saw how the TypeScript contract runs on `ckb-js-vm`, and how CCC is used in the frontend to generate lock-based addresses, query balances, build transactions, and send them.
- Understood the lock validation logic: load expected hash from script args, load preimage from witness, hash with `blake2b-256`, compare, return success (`0`) or failure (`11`).

## What I Understood

### Lock Script Authorization Model
I understood that ownership in this example is not tied to a private key signature directly, but to knowledge of a secret preimage. The lock only allows spending when the witness reveals a preimage whose hash matches the value embedded in `script_args`.

### Script Args and Witness Interaction
I learned that `script_args` hold static lock parameters (the expected hash), while witnesses provide dynamic proof during spending (the preimage). This made the separation between "lock condition" and "unlock proof" very clear.

### Address as Encoded Lock Script
I understood that a CKB address is essentially an encoded lock script. In the frontend, an address is generated from the custom lock script, and deposited funds are simply live Cells secured by that lock.

### Transaction Building for Unlocking
I learned that unlocking from `hash_lock` means consuming Cells locked by that script, creating new output Cells for the receiver, attaching the script cell dep, and inserting the preimage into witness args so the script can validate it.

### Security Limitation of Naive Hash Locks
I understood why this lock is only a learning example: once the preimage is revealed on-chain, others can reuse it. This creates front-running and post-reveal theft risks for remaining funds under the same hash lock.

## Key Takeaways
- A custom CKB lock can enforce arbitrary ownership rules beyond normal signature checks.
- `script_args` define lock parameters, and witnesses carry the unlock evidence at spend time.
- Building a real dApp on CKB includes both on-chain script deployment and frontend transaction construction.
- CCC and OffCKB provide a practical development path for local Devnet testing.
- Simple hash-preimage locks are useful for learning script mechanics but are not safe for production funds.

## Summary
Week 4 helped me connect script-level logic with actual dApp behavior. I learned how to design and deploy a custom lock, derive addresses from it, fund those addresses, and unlock funds by providing valid witness data. Most importantly, I gained a clearer understanding of how CKB lock scripts validate transaction intent and why security design matters before moving to production.
