# Weekly Progress Report - Week 6

## Overview
This week, I reviewed the CKB Script Development Course from Class 1 to Class 10.

My intention was not to follow every implementation detail exactly, because many of the course examples were written in C or JavaScript, while my main interest is the Rust scripting path. Instead, I went through all the classes, filtered out the parts that were mostly language-specific, and focused on the CKB scripting concepts that are still relevant when writing scripts in Rust.

The main goal was to understand how CKB scripts work at the validation level: how they inspect transactions, how lock and type scripts enforce rules, how UDTs are designed, how Type ID supports unique identities and upgrades, how debugging works, and why performance matters.

## Completed Learning

### 1. Class 1: Validation Model
- Reviewed the CKB validation model and how transactions consume existing Cells as inputs and create new Cells as outputs.
- Understood that CKB scripts validate state transitions instead of managing global contract storage like account-based blockchains.
- Learned that scripts act as rule checkers: if the transaction satisfies the rules, validation passes; otherwise, it fails.
- Picked this as very relevant to Rust because Rust scripts will still follow the same validation model.

### 2. Class 2: Script Basics
- Learned the basic structure of a CKB script: `code_hash`, `hash_type`, and `args`.
- Understood that script code is referenced on-chain and executed inside CKB-VM.
- Learned that a script succeeds when it returns `0` and fails when it returns a non-zero value.
- Picked the script structure and syscall-based transaction reading as relevant to Rust scripting.

### 3. Class 3: User Defined Token
- Studied how UDTs can be implemented using CKB's Cell model.
- Understood that a token transfer can be validated by comparing the total token amount in input Cells with the total token amount in output Cells.
- Learned that initial issuance needs a special rule so the token can be created once.
- Picked the UDT validation pattern as relevant because the same logic can be implemented in Rust by loading input and output Cells, parsing token amounts, and enforcing conservation rules.

### 4. Class 4: WebAssembly on CKB
- Reviewed how WebAssembly can be used in relation to CKB even though CKB-VM is based on RISC-V.
- Understood that CKB is flexible with language choices and execution paths.
- Treated this class mostly as background knowledge because my current path is Rust scripting rather than focusing on a WASM workflow.

### 5. Class 5: Debugging
- Learned that CKB scripts can be debugged locally using tools such as `ckb-debugger`.
- Understood the importance of reproducing the transaction environment when debugging scripts.
- Picked debugging as highly relevant to Rust because Rust scripts still need to be tested against real transaction inputs, outputs, Cell deps, witnesses, and script arguments.

### 6. Class 6: Type ID
- Studied Type ID as a way to create a unique identity for a Cell.
- Understood how Type ID can support script upgrade patterns by allowing script code to be referenced through a stable type identity.
- Learned the relationship between `hash_type = "type"` and referencing code through a dep Cell's type script hash.
- Picked Type ID as one of the most important concepts for future Rust script deployment.

### 7. Class 7: Advanced Duktape Examples
- Reviewed the advanced JavaScript/Duktape examples without focusing too deeply on the JavaScript implementation.
- Learned from the contract design patterns, especially the HTLC example.
- Understood how script arguments, witnesses, secrets, signatures, and time locks can combine to create more advanced spending conditions.
- Picked the transaction-design lessons as relevant, while leaving the Duktape-specific details as less important for my Rust path.

### 8. Class 8: Performant WASM
- Learned that performance is important on CKB because script execution consumes cycles.
- Reviewed how different compilation paths can affect binary size and cycle usage.
- Understood that lower-level compiled languages generally perform better than dynamic language runtimes.
- Picked this as support for my Rust direction, since Rust can provide strong performance while being safer and more maintainable than C.

### 9. Class 9: Cycle Reductions in Duktape Script
- Studied how cycle usage can be reduced in Duktape-based scripts.
- Learned that dynamic languages can work on CKB but may require heavy optimization to become practical.
- Understood that even after optimization, lower-level compiled scripts are usually much cheaper in cycles.
- Picked the general performance lesson as relevant: when writing Rust scripts, cycle efficiency should still be considered from the beginning.

### 10. Class 10: Language Choices
- Reviewed the different language options available for CKB scripting.
- Understood that CKB is flexible because scripts can be written in different languages as long as they compile or run in a form compatible with CKB-VM.
- Learned that the best language choice depends on the use case, maturity of tooling, safety, performance, and developer experience.
- Picked Rust as the most suitable direction for me because it balances safety, performance, and long-term maintainability.

## What I Understood

### Rust Path Over C Syntax
I understood that I do not need to master all the C examples line by line to benefit from the course. The important part is understanding the CKB concepts underneath the examples. Those concepts can still be applied when writing Rust scripts.

### Scripts as Transaction Validators
I understood that CKB scripts are not like traditional smart contracts with global mutable storage. They validate whether a transaction is allowed to transform old Cells into new Cells. This makes the Cell model the foundation of everything: transfers, tokens, custom locks, digital objects, and advanced contracts.

### Lock Scripts and Type Scripts
I reinforced the difference between lock scripts and type scripts. Lock scripts control who can spend a Cell, while type scripts control how a Cell's state can change. This distinction is very important for designing Rust scripts correctly.

### Script Args and Witnesses
I understood that `args` usually carry stable configuration for the script, while witnesses carry transaction-specific proof data. This pattern appeared across different classes and is directly relevant to Rust development.

### UDT and One-Time Issuance
The UDT class helped me understand how token rules can be enforced on CKB. The clever part was the initial issuance pattern, where the first input `OutPoint` is used as part of the script argument. Since that input can only be consumed once, it creates a unique token identity that cannot be recreated.

This immediately reminded me of one-shot minting on Cardano. It felt like CKB was saying, "You get one clean shot to create this token identity, so make it count." Once that specific input is spent, the unique reference is gone forever. That made the idea easier to understand because it connects well with the Cardano pattern of using a specific UTxO to guarantee uniqueness.

### Type ID and Upgradeability
I understood that Type ID is very important for real-world deployments. It gives a Cell a unique identity and allows code to be referenced in a more stable way. This can support upgrade patterns where the code changes, but the identity used to locate the code remains consistent.

### Debugging and Performance
I learned that debugging and performance are not optional parts of CKB development. Scripts need to be tested locally with realistic transaction data, and cycle usage matters because inefficient scripts can become expensive or impractical.

## Key Takeaways
- The CKB Script Development Course is useful even if the examples are mostly in C or JavaScript.
- My focus is the Rust scripting path, so I filtered the course for reusable CKB concepts instead of language-specific syntax.
- The most relevant topics for me were the validation model, script structure, Cell model, lock scripts, type scripts, UDTs, Type ID, witnesses, debugging, and cycle efficiency.
- UDT issuance on CKB has a clever uniqueness pattern that reminded me of one-shot minting on Cardano.
- Rust appears to be a strong path for CKB scripting because it combines safety, performance, and maintainability.
- Understanding CKB's model matters more than memorizing one course language, because the same rules still apply when writing scripts in Rust.

## Summary
Week 6 was a concept-filtering week. I went through the full CKB Script Development Course from Class 1 to Class 10, but I approached it from the perspective of someone preparing to write Rust scripts. I focused on the ideas that transfer across languages: transaction validation, Cell-based state, script arguments, witnesses, UDT rules, Type ID, debugging, and performance.

Although the course used many C and JavaScript examples, I was able to extract the parts that matter for my own Rust path. This gave me a clearer foundation for moving forward with CKB scripting without getting stuck in implementation details that are not central to my current direction.
