/// Converts a string representation of a number to an equivalent integer number
///
/// # Usage
/// ```
/// use we_cdk::*;
///
/// #[action]
/// fn _constructor() {
///     let result: Integer = parse_int!("31337");
/// }
/// ```
#[macro_export]
macro_rules! parse_int {
    ($value:expr) => {{
        let (error, result) = wevm::v0::bindings::parse_int($value.as_ptr(), $value.len());
        error!(error);
        result
    }};
}

/// Converts a string representation of a boolean to an equivalent logical value
///
/// # Usage
/// ```
/// use we_cdk::*;
///
/// #[action]
/// fn _constructor() {
///     let result: Boolean = parse_bool!("true");
/// }
/// ```
#[macro_export]
macro_rules! parse_bool {
    ($value:expr) => {{
        let (error, result) = wevm::v0::bindings::parse_bool($value.as_ptr(), $value.len());
        error!(error);
        result
    }};
}

/// Converts an integer to a byte array
///
/// # Usage
/// ```
/// use we_cdk::*;
///
/// #[action]
/// fn _constructor() {
///     let result: Binary = to_bytes!(31337);
/// }
/// ```
#[macro_export]
macro_rules! to_bytes {
    ($value:expr) => {{
        let (error, ptr, len) = wevm::v0::bindings::to_bytes($value);
        error!(error);
        core::slice::from_raw_parts(ptr, len)
    }};
}

/// Converts a byte array to an integer number
///
/// # Usage
/// ```
/// use we_cdk::*;
///
/// #[action]
/// fn _constructor() {
///     let binary: Binary = to_bytes!(31337);
///     let result: Integer = to_int!(binary);
/// }
/// ```
#[macro_export]
macro_rules! to_int {
    ($value:expr) => {{
        let (error, result) = wevm::v0::bindings::to_int($value.as_ptr(), $value.len());
        error!(error);
        result
    }};
}

/// Converts a logical value to a string
///
/// # Usage
/// ```
/// use we_cdk::*;
///
/// #[action]
/// fn _constructor() {
///     let result: String = to_string_bool!(true);
/// }
/// ```
#[macro_export]
macro_rules! to_string_bool {
    ($value:expr) => {{
        let (error, ptr, len) = wevm::v0::bindings::to_string_bool($value);
        error!(error);
        let bytes = core::slice::from_raw_parts(ptr, len);
        core::str::from_utf8_unchecked(bytes)
    }};
}

/// Converts an integer to a string
///
/// # Usage
/// ```
/// use we_cdk::*;
///
/// #[action]
/// fn _constructor() {
///     let result: String = to_string_int!(31337);
/// }
/// ```
#[macro_export]
macro_rules! to_string_int {
    ($value:expr) => {{
        let (error, ptr, len) = wevm::v0::bindings::to_string_int($value);
        error!(error);
        let bytes = core::slice::from_raw_parts(ptr, len);
        core::str::from_utf8_unchecked(bytes)
    }};
}
