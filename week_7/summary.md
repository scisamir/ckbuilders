# Weekly Progress Report - Week 7

## Overview
This week, I focused on the Rust scripting path for Nervos CKB.

After reviewing the broader CKB Script Development Course in Week 6, my goal this week was to move closer to the actual Rust developer workflow. I studied how Rust contracts are created, built, debugged, structured with `ckb-std`, tested locally before being used on-chain, and how CKB handles stable serialization through Molecule.

The main focus was understanding what it feels like to write CKB scripts as Rust programs that run inside CKB-VM, while still respecting the constraints of the Cell model, no-std execution, transaction validation, and cycle limits.

## Completed Learning

### 1. Rust Quick Start
- Reviewed the Rust Quick Start guide for CKB script development.
- Learned how a Rust script project can be generated using `ckb-script-templates`.
- Understood the basic project layout, including the contracts directory, tests project, Makefile, and helper scripts.
- Practiced the beginner workflow of generating a contract, building it, and running a simple script.
- Reinforced that Rust contracts need to work in a `no_std` environment because they run inside CKB-VM rather than a normal operating system.

Reference: https://docs.nervos.org/docs/script/rust/rust-quick-start

### 2. Building Rust Scripts
- Studied how Rust scripts are compiled for the CKB environment.
- Learned that modern Rust now has solid RISC-V support, making the build process more direct than older Capsule-based workflows.
- Understood the importance of compiling for the `riscv64imac-unknown-none-elf` target.
- Learned why release builds are strongly preferred for scripts: debug builds can be too large and may behave differently from release builds.
- Reviewed the idea of reproducible builds and why deterministic contract binaries matter for security.

Reference: https://docs.nervos.org/docs/script/rust/rust-build

### 3. Debugging Rust Scripts
- Practiced the debugging flow for Rust scripts using local tools instead of deploying directly on-chain.
- Reviewed how `ckb-debugger` can execute a compiled script and show script logs, return codes, and cycle usage.
- Understood that simple scripts can be run directly, while transaction-dependent scripts need a transaction file.
- Learned how `ckb-testtool` can help generate realistic transaction data that can later be inspected with `ckb-debugger`.
- Reinforced the value of `debug!` logging when checking script behavior.

Reference: https://docs.nervos.org/docs/script/rust/rust-debug

### 4. Rust API Introduction
- Studied the introduction to the Rust API through `ckb-std`.
- Understood that CKB scripts run in a bare-metal environment, so they do not have normal OS features like standard I/O, threads, or ordinary memory management.
- Learned that `ckb-std` provides the Rust-side foundation for writing scripts in this environment.
- Reviewed the main areas of the API, including entry setup, allocator setup, syscalls, error handling, logging, utilities, Type ID helpers, spawn, IPC, and native simulation.
- Connected the API structure back to the concepts from Week 6: scripts inspect transaction data through CKB syscalls and return success or failure based on validation rules.

Reference: https://docs.nervos.org/docs/script/rust/rust-api-introduction

### 5. Testing Rust Scripts
- Reviewed how Rust script tests are written with `ckb-testtool`.
- Learned that testing CKB scripts means building a local transaction environment that closely resembles on-chain execution.
- Practiced the structure of a basic test: deploy the contract binary, build scripts, prepare input and output Cells, construct a transaction, complete it with CellDeps, and verify it with a cycle limit.
- Understood why testing is especially important for CKB contracts because scripts protect assets and validate state transitions.
- Learned that tests can also cover edge cases that would be expensive or inconvenient to reproduce on-chain.

Reference: https://docs.nervos.org/docs/script/rust/rust-test

### 6. Serialization and Molecule in CKB
- Reviewed how serialization works in CKB and why stable encoding matters.
- Learned that serialization converts data structures into a format suitable for storage, transmission, and later reconstruction.
- Understood that CKB uses JSON mainly around RPC services, while Molecule is the core serialization format for CKB structures.
- Learned that Molecule is designed for canonical byte representation, which helps different implementations produce consistent results.
- Studied why Molecule is useful for CKB: it supports partial reading, self-contained substructures, and zero-copy access.
- Connected Molecule back to Rust scripting because scripts often need to read and validate structured data from Cells, witnesses, args, or transaction fields.

Reference: https://docs.nervos.org/docs/serialization/serialization-molecule-in-ckb

## What I Understood

### Rust Scripts Are Real CKB Contracts
I understood that writing scripts in Rust does not change the core CKB model. The script still validates transactions by reading Cells, script arguments, witnesses, and other transaction data. Rust mainly gives a safer and more maintainable way to express those validation rules.

### The Runtime Environment Is Different
I learned that CKB scripts do not run like normal Rust applications. They run in CKB-VM, target RISC-V, and operate without the normal standard library. This makes `ckb-std`, entry macros, allocators, syscalls, and careful binary size management very important.

### Build Configuration Matters
I understood that the build target is not just a technical detail. The script must be compiled for the right RISC-V target so it can run inside CKB-VM. I also learned that release builds are the practical default because script size, performance, and cycle usage matter.

### Debugging Needs Transaction Context
I learned that debugging a CKB script is not only about running the binary. For scripts that depend on transaction data, the script needs a realistic transaction context. This helped me understand why `ckb-testtool` and `ckb-debugger` are often used together.

### Testing Is Part of Contract Design
I understood that tests are not just a final check. In CKB development, tests help define and validate the transaction shape itself: which Cells exist, which scripts are attached, which witnesses are provided, and what state transition is allowed.

### ckb-std Is the Rust Bridge Into CKB
The Rust API introduction made it clearer that `ckb-std` is the main bridge between Rust code and the CKB execution environment. It provides access to script entry setup, syscalls, logging, errors, utility helpers, and higher-level abstractions that make Rust scripting practical.

### Molecule Gives CKB Stable Data Meaning
I understood that CKB scripts often work with raw bytes, but those bytes still need a stable structure. Molecule helps define that structure in a canonical way so different tools, SDKs, and scripts can agree on the exact byte representation.

This is important because smart contract validation depends on precision. If two implementations serialize the same logical data differently, script behavior could become unreliable. Molecule reduces that risk by giving CKB a consistent format for core structures and custom data.

## Key Takeaways
- Rust scripting on CKB follows the same validation model learned in previous weeks.
- `ckb-script-templates` provides a practical starting point for creating Rust script projects.
- Rust scripts must be compiled for the `riscv64imac-unknown-none-elf` target.
- Release builds are strongly preferred because binary size, performance, and cycle usage matter.
- `ckb-std` provides the key Rust APIs needed in the no-std CKB-VM environment.
- `ckb-testtool` is useful for constructing realistic transaction tests.
- `ckb-debugger` is useful for inspecting script logs, return codes, transaction-based execution, and cycle usage.
- Molecule is central to CKB serialization because it gives data a stable, canonical byte format.
- Serialization matters for Rust scripts because scripts often parse structured Cell data, witnesses, args, and transaction fields.
- Good Rust script development requires building, testing, and debugging as one connected workflow.

## Summary
Week 7 was a practical Rust scripting week. I moved from general CKB script concepts into the Rust-specific workflow for creating, building, debugging, understanding APIs, testing CKB scripts, and thinking more carefully about serialized data.

The biggest improvement this week was understanding the complete local development loop: generate a Rust contract, compile it for the correct RISC-V target, test it with `ckb-testtool`, inspect behavior with `ckb-debugger`, and use `ckb-std` to interact with the CKB environment safely.

Adding Molecule also helped me understand that script development is not only about writing validation logic. It is also about making sure the data being validated has a stable and consistent format. This made the Rust path feel much more concrete. I now have a clearer picture of how to go from concept to executable CKB script, how to validate script behavior before deployment, and why serialization is part of reliable contract design.
