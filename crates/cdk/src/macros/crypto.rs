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
        let (error, ptr, len) = wevm::v0::bindings::fast_hash($bytes.as_ptr(), $bytes.len());
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
        let (error, ptr, len) = wevm::v0::bindings::secure_hash($bytes.as_ptr(), $bytes.len());
        error!(error);
        core::slice::from_raw_parts(ptr, len)
    }};
}

/// Hash an array of bytes using BLAKE2b-256 (https://en.wikipedia.org/wiki/BLAKE_%28hash_function%29)
///
/// # Usage
/// ```
/// use we_cdk::*;
///
/// #[action]
/// fn _constructor() {
///     let hash: Binary = blake2b256!("hello");
/// }
/// ```
#[macro_export]
macro_rules! blake2b256 {
    ($bytes:expr) => {{
        let (error, ptr, len) = wevm::v0::bindings::blake2b256($bytes.as_ptr(), $bytes.len());
        error!(error);
        core::slice::from_raw_parts(ptr, len)
    }};
}

/// Hash an array of bytes using Keccak-256 (https://keccak.team/files/Keccak-submission-3.pdf)
///
/// # Usage
/// ```
/// use we_cdk::*;
///
/// #[action]
/// fn _constructor() {
///     let hash: Binary = keccak256!("hello");
/// }
/// ```
#[macro_export]
macro_rules! keccak256 {
    ($bytes:expr) => {{
        let (error, ptr, len) = wevm::v0::bindings::keccak256($bytes.as_ptr(), $bytes.len());
        error!(error);
        core::slice::from_raw_parts(ptr, len)
    }};
}

/// Hash an array of bytes using SHA-256 (https://en.wikipedia.org/wiki/SHA-2)
///
/// # Usage
/// ```
/// use we_cdk::*;
///
/// #[action]
/// fn _constructor() {
///     let hash: Binary = sha256!("hello");
/// }
/// ```
#[macro_export]
macro_rules! sha256 {
    ($bytes:expr) => {{
        let (error, ptr, len) = wevm::v0::bindings::sha256($bytes.as_ptr(), $bytes.len());
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
///     let message = "uncle";
///     let signature: Binary = base58!("B4ViRpS6wZ73hhTtP4hhrfV46rR3uoUn7jgsH5yfkKMpbJUxMmu48jf3QSdibRkQBN7Tkx9jReKDq1Rmp9acxPG");
///     let public_key: Binary = base58!("4KxUVD9NtyRJjU3BCvPgJSttoJX7cb3DMdDTNucLN121");
///     let result: Boolean = sig_verify!(message, signature, public_key);
/// }
/// ```
#[macro_export]
macro_rules! sig_verify {
    ($message:expr, $signature:expr, $public_key: expr) => {{
        let (error, result) = wevm::v0::bindings::sig_verify(
            $message.as_ptr(),
            $message.len(),
            $signature.as_ptr(),
            $signature.len(),
            $public_key.as_ptr(),
            $public_key.len(),
        );
        error!(error);
        result
    }};
}
