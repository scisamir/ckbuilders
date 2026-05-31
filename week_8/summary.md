# Weekly Progress Report - Week 8

## Overview
This week, I started a fresh CKB smart contract project in Rust from scratch.

I generated a new CKB Rust workspace using `ckb-script-templates`, prepared the RISC-V target, created my first contract crate, fixed build issues related to the Rust and CKB toolchain, implemented a basic lock script validation rule, renamed the contract safely to `lock-script-contract`, and confirmed everything works by running the tests successfully.

I also learned the basic structure of a CKB Rust smart contract project: the root `Cargo.toml` manages the workspace members, each contract has its own `Cargo.toml`, the smart contract logic lives in `contracts/<contract-name>/src/main.rs`, and tests are written in `tests/src/tests.rs`.

## Completed Learning

### 1. Created a Fresh CKB Rust Workspace
- Generated a new smart contract workspace from the official `ckb-script-templates` project.
- Learned that the workspace template creates the main project structure needed for CKB Rust contract development.
- Practiced starting from a clean project instead of only reading existing examples.

Command used:

```bash
cargo generate --git https://github.com/cryptape/ckb-script-templates workspace --name <project-name>
```

### 2. Prepared the RISC-V Build Target
- Entered the new project directory and prepared the required RISC-V target.
- Reinforced that CKB smart contracts are compiled for CKB-VM, not for a normal desktop runtime.
- Understood that the Rust toolchain and target setup must be correct before contract builds can work.

Commands used:

```bash
cd <project-name>
make prepare
```

### 3. Generated a Contract Crate
- Created a new contract inside the workspace.
- Learned that each contract is its own crate under the `contracts` directory.
- Connected the generated contract layout to the workspace configuration in the root `Cargo.toml`.

Command used:

```bash
make generate CRATE=<contract-name>
```

### 4. Built the Contract
- Built the contract without using the native simulator.
- Learned the build command pattern for targeting a specific contract and task.
- Practiced reading build errors and connecting them back to toolchain configuration.

Command used:

```bash
make run CONTRACT=<contract-name> TASK=build
```

### 5. Fixed Rust and CKB Toolchain Build Issues
- Fixed the contract `Makefile` by adding `+forced-atomics` to the RISC-V target features.
- Learned that some Rust dependencies may expect atomic symbols that are not automatically available in the bare-metal CKB target.
- Added atomic shim functions in `main.rs` when the linker complained about missing symbols such as:

```text
__sync_fetch_and_add_8
__sync_fetch_and_sub_8
__sync_val_compare_and_swap_8
```

This helped me understand that CKB Rust contract development sometimes requires working directly with low-level build and linker details.

### 6. Implemented a Basic Lock Script Rule
- Wrote a simple validation rule inside `program_entry()`.
- Used script arguments as the condition checked by the lock script.
- Learned that a lock script succeeds or fails based on whether the transaction satisfies the validation logic.

### 7. Updated and Ran Tests
- Updated the test transaction so it matched the contract rule.
- Practiced using the test environment to validate the contract before deployment.
- Confirmed the contract behavior by running the test suite successfully.

Command used:

```bash
make test
```

### 8. Safely Renamed the Contract
- Renamed the contract to `lock-script-contract`.
- Learned that renaming a contract requires updating multiple project locations consistently.
- Updated the contract directory, workspace configuration, contract package metadata, and tests.

Files and paths updated:

```text
contracts/<old-name>
root Cargo.toml
contracts/<new-name>/Cargo.toml
tests/src/tests.rs
```

## What I Understood

### The Workspace Controls the Project
I understood that the root `Cargo.toml` is responsible for managing the workspace members. If a contract is added, removed, or renamed, the workspace configuration must stay in sync.

### Each Contract Is Its Own Rust Crate
I learned that contracts live under `contracts/<contract-name>` and each contract has its own `Cargo.toml`. This makes each smart contract a separate Rust package inside the larger workspace.

### Contract Logic Lives in main.rs
The smart contract validation logic is written in `contracts/<contract-name>/src/main.rs`. For this week, the main logic was a basic lock script rule inside `program_entry()`.

### Tests Must Match the Script Rule
I understood that tests need to build a transaction that satisfies the script's validation logic. If the script expects specific args, the test transaction must provide matching args.

### Build Errors Can Come From the Target Environment
The linker errors taught me that CKB Rust contracts run in a constrained target environment. Missing atomic symbols and RISC-V target features are not normal application bugs; they are part of making Rust code compile correctly for CKB-VM.

### Renaming a Contract Needs Care
I learned that renaming a contract is not just a folder rename. The workspace, package name, contract path, and tests all need to be updated together so the project still builds and tests correctly.

## Key Takeaways
- A fresh CKB Rust smart contract project can be generated with `ckb-script-templates`.
- `make prepare` sets up the RISC-V target needed for CKB contract builds.
- New contracts can be generated with `make generate CRATE=<contract-name>`.
- Contracts can be built with `make run CONTRACT=<contract-name> TASK=build`.
- The root `Cargo.toml` manages the contract crates in the workspace.
- Contract code lives in `contracts/<contract-name>/src/main.rs`.
- Tests live in `tests/src/tests.rs`.
- Toolchain issues may require updating RISC-V target features and adding atomic shim functions.
- A basic lock script can validate transaction data by checking script args.
- Tests must construct transactions that match the contract's validation rule.
- Contract renaming requires updating the directory, workspace config, contract package config, and tests.

## Final Result
The contract is now named:

```text
lock-script-contract
```

It builds successfully, the tests pass, and I now understand the clean workflow for creating, building, renaming, and testing a CKB Rust smart contract from scratch.

## Summary
Week 8 was a hands-on Rust smart contract setup week. I moved from studying the Rust CKB workflow to actually creating a new project, generating a contract, fixing build issues, writing a simple lock script rule, updating tests, and confirming everything works.

The biggest improvement this week was learning the full practical loop: generate the workspace, prepare the target, generate the contract, build it, fix toolchain problems, implement validation logic, update tests, run the tests, and safely rename the contract. This made the CKB Rust development workflow feel much more concrete and repeatable.
