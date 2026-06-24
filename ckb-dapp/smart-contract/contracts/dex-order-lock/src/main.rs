#![cfg_attr(not(any(feature = "library", test)), no_std)]
#![cfg_attr(not(test), no_main)]

#[cfg(any(feature = "library", test))]
extern crate alloc;

#[cfg(not(any(feature = "library", test)))]
ckb_std::entry!(program_entry);
#[cfg(not(any(feature = "library", test)))]
// By default, the following heap configuration is used:
// * 16KB fixed heap
// * 1.2MB(rounded up to be 16-byte aligned) dynamic heap
// * Minimal memory block in dynamic heap is 64 bytes
// For more details, please refer to ckb-std's default_alloc macro
// and the buddy-alloc alloc implementation.
ckb_std::default_alloc!(16384, 1258306, 64);

pub fn program_entry() -> i8 {
    let script = match ckb_std::high_level::load_script() {
        Ok(script) => script,
        Err(err) => {
            ckb_std::debug!("load script failed: {:?}", err);
            return -1;
        }
    };

    let args = script.args().raw_data();

    if args.len() != 40 {
        ckb_std::debug!("expected 40-byte args, got {}", args.len());
        return -2;
    }

    let args_slice: &[u8] = args.as_ref();
    let (maker_bytes, price_bytes) = args_slice.split_at(32);

    let maker_lock_hash: [u8; 32] = match maker_bytes.try_into() {
        Ok(bytes) => bytes,
        Err(_) => return -3,
    };

    let price_array: [u8; 8] = match price_bytes.try_into() {
        Ok(bytes) => bytes,
        Err(_) => return -4,
    };

    let ask_price = u64::from_le_bytes(price_array);

    0
}
