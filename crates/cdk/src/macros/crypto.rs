/// Fast hash
///
/// # Usage
/// ```
/// use we_cdk::*;
///
/// #[action]
/// fn _constructor() {
///     let hash: Binary = fast_hash!("hello");
/// }
/// ```
#[macro_export]
macro_rules! fast_hash {
    ($bytes:expr) => {{
        let (error, ptr, len) = bindings::v0::fast_hash($bytes.as_ptr(), $bytes.len());
        error!(error);
        core::slice::from_raw_parts(ptr, len)
    }};
}

/// Secure hash
///
/// # Usage
/// ```
/// use we_cdk::*;
///
/// #[action]
/// fn _constructor() {
///     let hash: Binary = secure_hash!("hello");
/// }
/// ```
#[macro_export]
macro_rules! secure_hash {
    ($bytes:expr) => {{
        let (error, ptr, len) = bindings::v0::secure_hash($bytes.as_ptr(), $bytes.len());
        error!(error);
        core::slice::from_raw_parts(ptr, len)
    }};
}

/// Signature verify
///
/// # Usage
/// ```
/// use we_cdk::*;
///
/// #[action]
/// fn _constructor() {
///     let hash: Binary = fast_hash!("6Tn7ir9MycHW6Gq2F2dGok2stokSwXJadPh4hW8eZ8Sp");
/// }
/// ```
#[macro_export]
macro_rules! sig_verify {
    ($message:expr, $signature:expr, $public_key: expr) => {{
        let (error, ptr, len) = bindings::v0::sig_verify(
            $message.as_ptr(),
            $message.len(),
            $signature.as_ptr(),
            $signature.len(),
            $public_key.as_ptr(),
            $public_key.len(),
        );
        error!(error);
        core::slice::from_raw_parts(ptr, len)
    }};
}
