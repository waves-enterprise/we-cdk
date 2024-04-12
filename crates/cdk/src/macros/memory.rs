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
        let (error, result) = wevm::v0::bindings::binary_equals(
            $left.as_ptr(),
            $left.len(),
            $right.as_ptr(),
            $right.len(),
        );
        error!(error);
        result
    }};
    (string :: $left:expr, $right:expr) => {{
        let (error, result) = wevm::v0::bindings::string_equals(
            $left.as_ptr(),
            $left.len(),
            $right.as_ptr(),
            $right.len(),
        );
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
        let (error, ptr, len) = wevm::v0::bindings::join(
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

/// Checks if the string/binary is contained in a string/binary
///
/// # Usage
/// ```
/// use we_cdk::*;
///
/// #[action]
/// fn _constructor() {
///     let result: Boolean = contains!(&[0, 1, 2, 3, 4, 5], &[2, 3]);
///     let result: Boolean = contains!("Hello, world!", "world");
/// }
/// ```
#[macro_export]
macro_rules! contains {
    ($bytes:expr, $subbytes:expr) => {{
        let (error, result) = wevm::v0::bindings::contains(
            $bytes.as_ptr(),
            $bytes.len(),
            $subbytes.as_ptr(),
            $subbytes.len(),
        );
        error!(error);
        result
    }};
}

/// Returns a string/binary without the first N characters/bytes
///
/// # Usage
/// ```
/// use we_cdk::*;
///
/// #[action]
/// fn _constructor() {
///     let result: Binary = drop!(&[0, 1, 2, 3, 4, 5], 1);
/// }
/// ```
#[macro_export]
macro_rules! drop {
    ($bytes:expr, $n:expr) => {{
        let (error, ptr, len) = wevm::v0::bindings::drop($bytes.as_ptr(), $bytes.len(), $n);
        error!(error);
        core::slice::from_raw_parts(ptr, len)
    }};
}

/// Returns a string/binary without the last N characters/bytes
///
/// # Usage
/// ```
/// use we_cdk::*;
///
/// #[action]
/// fn _constructor() {
///     let result: Binary = drop_right!(&[0, 1, 2, 3, 4, 5], 1);
/// }
/// ```
#[macro_export]
macro_rules! drop_right {
    ($bytes:expr, $n:expr) => {{
        let (error, ptr, len) = wevm::v0::bindings::drop_right($bytes.as_ptr(), $bytes.len(), $n);
        error!(error);
        core::slice::from_raw_parts(ptr, len)
    }};
}

/// Returns the index of the first occurrence of the substring (if no occurrence is found, returns -1)
///
/// # Usage
/// ```
/// use we_cdk::*;
///
/// #[action]
/// fn _constructor() {
///     let result: Integer = index_of!("Hello, world!", "world");
/// }
/// ```
#[macro_export]
macro_rules! index_of {
    ($string:expr, $substring:expr) => {{
        let (error, result) = wevm::v0::bindings::index_of(
            $string.as_ptr(),
            $string.len(),
            $substring.as_ptr(),
            $substring.len(),
        );
        error!(error);
        result
    }};
}

/// Returns the index of the last occurrence of the substring (if no occurrence is found, returns -1)
///
/// # Usage
/// ```
/// use we_cdk::*;
///
/// #[action]
/// fn _constructor() {
///     let result: Integer = last_index_of!("Hello, world!", "world");
/// }
/// ```
#[macro_export]
macro_rules! last_index_of {
    ($string:expr, $substring:expr) => {{
        let (error, result) = wevm::v0::bindings::last_index_of(
            $string.as_ptr(),
            $string.len(),
            $substring.as_ptr(),
            $substring.len(),
        );
        error!(error);
        result
    }};
}

/// Returns the first N characters/bytes of the string/binary
///
/// # Usage
/// ```
/// use we_cdk::*;
///
/// #[action]
/// fn _constructor() {
///     let result: Binary = take!(&[0, 1, 2, 3, 4, 5], 1);
/// }
/// ```
#[macro_export]
macro_rules! take {
    ($bytes:expr, $n:expr) => {{
        let (error, ptr, len) = wevm::v0::bindings::take($bytes.as_ptr(), $bytes.len(), $n);
        error!(error);
        core::slice::from_raw_parts(ptr, len)
    }};
}

/// Returns a string/binary without the last N characters/bytes
///
/// # Usage
/// ```
/// use we_cdk::*;
///
/// #[action]
/// fn _constructor() {
///     let result: Binary = take_right!(&[0, 1, 2, 3, 4, 5], 1);
/// }
/// ```
#[macro_export]
macro_rules! take_right {
    ($bytes:expr, $n:expr) => {{
        let (error, ptr, len) = wevm::v0::bindings::take_right($bytes.as_ptr(), $bytes.len(), $n);
        error!(error);
        core::slice::from_raw_parts(ptr, len)
    }};
}
