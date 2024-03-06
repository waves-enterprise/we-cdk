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
///     let address = base58!("3NzkzibVRkKUzaRzjUxndpTPvoBzQ3iLng3");
/// }
/// ```
#[macro_export]
macro_rules! base58 {
    ($value:expr) => {{
        let (error, ptr, len) = bindings::v0::base_58($value.as_ptr(), $value.len());
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
///     let address = base58!("3NzkzibVRkKUzaRzjUxndpTPvoBzQ3iLng3");
///     let address_string: String = to_base58_string!(address);
/// }
/// ```
#[macro_export]
macro_rules! to_base58_string {
    ($value:expr) => {{
        let (error, ptr, len) = bindings::v0::to_base_58_string($value.as_ptr(), $value.len());
        error!(error);
        let bytes = core::slice::from_raw_parts(ptr, len);
        core::str::from_utf8_unchecked(bytes)
    }};
}

/// Comparison data of Binary and String types
///
/// # Usage
/// ```
/// use we_cdk::*;
///
/// #[action]
/// fn _constructor() {
///     let binary_equals: Boolean = equals!(binary :: &[0, 1], &[0, 1]);
///     let string_equals: Boolean = equals!(string :: "test", "value");
/// }
/// ```
#[macro_export]
macro_rules! equals {
    (binary :: $left:expr, $right:expr) => {{
        let (error, result) =
            bindings::v0::binary_equals($left.as_ptr(), $left.len(), $right.as_ptr(), $right.len());
        error!(error);
        result
    }};
    (string :: $left:expr, $right:expr) => {{
        let (error, result) =
            bindings::v0::string_equals($left.as_ptr(), $left.len(), $right.as_ptr(), $right.len());
        error!(error);
        result
    }};
}

/// Joining data of Binary and String types
///
/// # Usage
/// ```
/// use we_cdk::*;
///
/// #[action]
/// fn _constructor() {
///     let bytes: Binary = join!(binary :: &[0, 1], &[2, 3]);
///     let hello_world: String = join!(string :: "Hello", ", ", "world", "!");
/// }
/// ```
#[macro_export]
macro_rules! join {
    // For use within a macro
    (@inner, $temp:expr, $value:expr) => {
        let (error, ptr, len) = bindings::v0::join(
            $temp.as_ptr(),
            $temp.len(),
            $value.as_ptr(),
            $value.len()
        );
        error!(error);
        $temp = core::slice::from_raw_parts(ptr, len);
    };
    (binary :: $( $value:expr ),+ ) => {{
        let mut temp: &[u8] = &[0u8; 0];
        $( join!(@inner, temp, $value); )+
        temp
    }};
    (string :: $( $value:expr ),+ ) => {{
        let mut temp: &[u8] = &[0u8; 0];
        $( join!(@inner, temp, $value); )+
        core::str::from_utf8_unchecked(temp)
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
///     let caller = caller!();
/// }
/// ```
#[macro_export]
macro_rules! caller {
    () => {{
        let (error, ptr, len) = bindings::v0::caller();
        error!(error);
        core::slice::from_raw_parts(ptr, len)
    }};
}
