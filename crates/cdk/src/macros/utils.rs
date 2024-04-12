/// Base58 string to bytes conversion
///
/// # Result
/// The result of execution is `&[u8]` bytes
///
/// # Usage
/// ```
/// use we_cdk::*;
///
/// #[action]
/// fn _constructor() {
///     let address: Binary = base58!("3NzkzibVRkKUzaRzjUxndpTPvoBzQ3iLng3");
/// }
/// ```
#[macro_export]
macro_rules! base58 {
    ($value:expr) => {{
        let (error, ptr, len) = wevm::v0::bindings::base_58($value.as_ptr(), $value.len());
        error!(error);
        core::slice::from_raw_parts(ptr, len)
    }};
}

/// Bytes to Base58 string conversion
///
/// # Result
/// The result of execution is `&[u8]` bytes
///
/// # Usage
/// ```
/// use we_cdk::*;
///
/// #[action]
/// fn _constructor() {
///     let address: Binary = base58!("3NzkzibVRkKUzaRzjUxndpTPvoBzQ3iLng3");
///     let address_string: String = to_base58_string!(address);
/// }
/// ```
#[macro_export]
macro_rules! to_base58_string {
    ($value:expr) => {{
        let (error, ptr, len) =
            wevm::v0::bindings::to_base_58_string($value.as_ptr(), $value.len());
        error!(error);
        let bytes = core::slice::from_raw_parts(ptr, len);
        core::str::from_utf8_unchecked(bytes)
    }};
}

/// Get the ContractId of the calling contract
///
/// # Result
/// The result of execution is `&[u8]` ContractId
/// If the returned slice is empty, then the contract was called from a transaction
///
/// # Usage
/// ```
/// use we_cdk::*;
///
/// #[action]
/// fn _constructor() {
///     let caller: Binary = caller!();
/// }
/// ```
#[macro_export]
macro_rules! caller {
    () => {{
        let (error, ptr, len) = wevm::v0::bindings::caller();
        error!(error);
        core::slice::from_raw_parts(ptr, len)
    }};
}
