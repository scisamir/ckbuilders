# Weekly Progress Report - Week 9

## Overview

This week, I started building a fixed-price DEX order lock on Nervos CKB using
Rust.

Instead of trying to complete the entire DEX contract immediately, I focused on
understanding the transaction design and building a correct foundation. I
visualized the order lifecycle, separated the responsibilities of the DEX lock
and xUDT type script, generated a new Rust contract, designed the lock argument
format, implemented argument parsing, and confirmed that valid order arguments
pass contract verification.

The main goal was to understand every piece before adding settlement and
cancellation rules. This made Week 9 the design and contract-bootstrapping phase
of the DEX project. The remaining validation logic will continue in Week 10.

## Completed Learning

### 1. Designed a Fixed-Price DEX Order

- Designed a simple order-based DEX instead of a more complicated AMM.
- Limited the first version to selling one typed asset, intended to be an xUDT,
  for a fixed amount of CKB.
- Kept the design intentionally small:
  - Fixed price
  - Full fills only
  - CKB payment
  - One DEX order per transaction
  - Maker-authorized cancellation
- Created flowcharts showing the order creation, fulfillment, and cancellation
  paths.
- Documented the design in `week_9/design.md` as the single source of truth.

### 2. Understood the Order Cell Structure

- Defined the Order Cell as a Cell containing:
  - Capacity for on-chain storage
  - The DEX order lock
  - An xUDT type script
  - The token amount in Cell data
- Understood that the DEX lock and xUDT type script have different
  responsibilities.
- The DEX lock protects the trade conditions.
- The xUDT type script protects token accounting.
- The maker's normal lock script will authenticate cancellation.

### 3. Generated a New Rust Contract

- Preserved the Week 8 learning contract and generated a separate contract
  named:

```text
dex-order-lock
```

- Confirmed that the new contract was added to the Rust workspace.
- Built the generated contract successfully for the CKB RISC-V target.
- Fixed the executable permission of the `scripts/find_clang` helper so the
  build system could locate Clang normally.

Commands used:

```bash
cd ckb-dapp/smart-contract
make generate CRATE=dex-order-lock
chmod +x scripts/find_clang
make run CONTRACT=dex-order-lock TASK=build
```

### 4. Designed the Lock Argument Format

- Defined the DEX lock arguments as exactly 40 bytes.
- Used the first 32 bytes for the maker's lock script hash.
- Used the final 8 bytes for the fixed ask price.
- Chose to store the price in shannons as a little-endian `u64`.

The argument layout is:

```text
Bytes 0-31  = Maker lock hash
Bytes 32-39 = Ask price in shannons
Total       = 40 bytes
```

- Reinforced that:

```text
1 CKB = 100,000,000 shannons
```

- Understood that using an integer amount in shannons avoids floating-point and
  rounding problems.

### 5. Loaded the Currently Executing Script

- Used `ckb_std::high_level::load_script()` to load the script currently being
  executed inside CKB-VM.
- Understood that the returned `Script` contains:
  - `code_hash`
  - `hash_type`
  - `args`
- Learned that the DEX executable is located through a CellDep, while
  `load_script()` returns the current script description from the transaction
  validation context.
- Extracted the raw lock arguments using:

```text
script.args().raw_data()
```

### 6. Parsed the Maker Lock Hash and Ask Price

- Validated that the current script contains exactly 40 argument bytes.
- Converted the argument data into a borrowed byte slice.
- Split the bytes at index 32 to separate the maker lock hash from the price.
- Converted the maker bytes into a fixed `[u8; 32]` array.
- Converted the price bytes into a fixed `[u8; 8]` array.
- Decoded the price with `u64::from_le_bytes()`.
- Added distinct non-zero return codes for script-loading, argument-length, and
  conversion failures.

### 7. Strengthened My Rust Fundamentals

This implementation helped me understand several Rust concepts more clearly:

- `u8` represents one unsigned byte.
- `u64` represents a 64-bit unsigned integer.
- Signed integers can represent negative values, while unsigned integers cannot.
- `[u8; 32]` is a fixed array containing exactly 32 bytes.
- `&[u8]` is a borrowed, read-only view over byte data.
- A slice can view either part of a collection or the entire collection.
- `as_ref()` can borrow the underlying bytes without copying them.
- `split_at()` divides one slice into two borrowed slices.
- `try_into()` checks whether a runtime-length slice can become a fixed-size
  array.
- `Vec<u8>` is useful when constructing serialized data because it can grow.
- `to_le_bytes()` serializes an integer into little-endian bytes.
- `from_le_bytes()` reconstructs the integer from those bytes.

### 8. Constructed Valid Arguments in the Test

- Created a dummy 32-byte maker lock hash for the test.
- Defined a 500 CKB ask price as:

```text
50,000,000,000 shannons
```

- Converted the maker array into a growable byte vector.
- Converted the ask price into eight little-endian bytes.
- Appended both fields to produce the expected 40-byte lock arguments.
- Added an assertion confirming that the final argument length is 40.
- Passed the arguments to the generated DEX lock script in the test transaction.

### 9. Built and Verified the Argument Parser

- Rebuilt the contract after implementing the argument parser.
- Ran the targeted `ckb-testtool` test successfully.
- Confirmed that valid 40-byte order arguments pass verification.
- Recorded the current successful execution cost:

```text
17,624 cycles
```

Command used:

```bash
cargo test -p tests test_dex_order_lock -- --nocapture
```

## What I Understood

### A DEX Order Can Be Represented as a Cell

I understood that this DEX does not need a global order book inside the
contract. The maker creates an Order Cell containing the asset and attaches the
DEX lock. That Cell becomes the maker's standing offer.

Anyone can attempt to consume the Order Cell, but the DEX lock will eventually
allow the transaction only when the maker receives the required payment or
authorizes a cancellation.

### Script Arguments Are Static Contract Configuration

The maker lock hash and ask price belong in the lock arguments because they are
the fixed conditions of one order.

The token ID and amount do not need to be duplicated in the DEX arguments. The
asset identity belongs to the Cell's type script, and the amount belongs to the
xUDT Cell data.

### Raw Arguments Must Be Given Meaning

I understood that CKB provides script arguments as raw bytes. The contract must
define and consistently follow its own byte layout.

The first 32 bytes do not automatically become a lock hash, and the final 8
bytes do not automatically become a number. The contract gives those bytes
meaning by splitting, converting, and decoding them.

### Arrays, Slices, and Vectors Have Different Roles

I learned that fixed arrays are useful when a field must have an exact size,
slices are useful for borrowing and inspecting raw data, and vectors are useful
when constructing a byte sequence that needs to grow.

This made the encoding and decoding process clearer:

```text
Test:
Maker array + price bytes -> Vec<u8> containing 40 bytes

Contract:
40-byte slice -> maker array + price array -> u64 price
```

### Little-Endian Must Match on Both Sides

I understood that little-endian is the agreed encoding format for this
contract. The test uses `to_le_bytes()` to serialize the price, and the contract
uses `from_le_bytes()` to decode it.

Big-endian could also work in a custom format, but both sides must use the same
convention. Little-endian keeps this design consistent with common CKB numeric
encoding.

### Contract Responsibilities Should Remain Separate

I understood that the DEX lock should not reimplement xUDT token rules or the
maker's signature verification.

The intended composition is:

```text
DEX lock          -> validates trade settlement
xUDT type script  -> validates token accounting
Maker lock        -> authenticates cancellation
```

This separation keeps each script focused on one responsibility.

### The First Milestone Is Not Yet a Complete DEX

The current contract correctly validates and decodes its order configuration,
but it does not yet validate payment or cancellation.

I deliberately split the work so the foundation could be understood and tested
before adding transaction settlement rules. Week 10 will complete the actual
DEX validation paths.

## Key Takeaways

- A simple fixed-price order is a manageable first DEX design for CKB.
- An Order Cell can hold the asset while its lock script defines settlement
  conditions.
- The DEX lock, xUDT type script, and maker lock each have separate
  responsibilities.
- The DEX lock arguments contain exactly one source for the maker identity and
  price.
- Script arguments are raw bytes and require an explicit encoding format.
- The maker lock hash uses 32 bytes, while the ask price uses an 8-byte
  little-endian `u64`.
- `load_script()` returns the script currently executing inside CKB-VM.
- Rust arrays, slices, and vectors are useful for different stages of binary
  data handling.
- `try_into()` converts runtime-length slices into fixed-size arrays only when
  the lengths match.
- Valid 40-byte DEX arguments now pass the local contract test.
- Settlement, cancellation, typed-asset checks, and negative tests remain for
  Week 10.

## Final Result

The new contract is named:

```text
dex-order-lock
```

It builds successfully, loads and validates its 40-byte script arguments,
extracts the maker lock hash, decodes the fixed ask price, and passes the current
positive test.

The successful test currently consumes:

```text
17,624 cycles
```

## Next Steps

Week 10 will continue the same contract by:

- Detecting maker-authorized cancellation
- Requiring a typed asset for order fulfillment
- Restricting settlement to one DEX order per transaction
- Loading the Order Cell capacity
- Safely calculating the required maker payment
- Summing CKB outputs sent to the maker
- Rejecting underpayment and incorrect recipients
- Adding successful-fill, cancellation, and failure tests
- Recording the final cycle usage

## Summary

Week 9 was the design and bootstrapping phase of the fixed-price DEX project.

I moved from the general idea of a DEX into a concrete CKB Cell-based design,
created a dedicated Rust contract, defined a minimal order format, and
implemented the first contract boundary: loading and decoding the order's
configuration safely.

The most important improvement this week was not only getting the parser to
work, but understanding why each representation exists. I learned how raw CKB
script arguments become Rust slices, fixed arrays, hashes, and integer values,
and how the test performs the reverse encoding process.

The contract is not yet a complete DEX, but it now has a clear design, a
working Rust foundation, a single source of truth for its argument layout, and
a passing local test. This creates a clean starting point for implementing
settlement and cancellation in Week 10.
