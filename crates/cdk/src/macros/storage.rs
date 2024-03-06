/// Get storage
///
/// # Usage
/// ```
/// use we_cdk::*;
///
/// #[action]
/// fn _constructor() {
///     let address = base58!("3NzkzibVRkKUzaRzjUxndpTPvoBzQ3iLng3");
///
///     let integer_value: Integer = get_storage!(integer :: "integer_key");
///     let boolean_value: Boolean = get_storage!(boolean :: "boolean_key");
///     let binary_value: Binary = get_storage!(binary :: "binary_key");
///     let string_value: String = get_storage!(string :: "string_key");
///
///     let address_int_value = get_storage!(integer :: address => "integer_key");
/// }
/// ```
#[macro_export]
macro_rules! get_storage {
    (integer :: $key:expr) => {{
        let this = internal_data!(this);
        let (error, value) =
            bindings::v0::get_storage_int(this.0, this.1, $key.as_ptr(), $key.len());
        error!(error);
        value
    }};
    (integer :: $address:expr => $key:expr) => {{
        let (error, value) = bindings::v0::get_storage_int(
            $address.as_ptr(),
            $address.len(),
            $key.as_ptr(),
            $key.len(),
        );
        error!(error);
        value
    }};
    (boolean :: $key:expr) => {{
        let this = internal_data!(this);
        let (error, value) =
            bindings::v0::get_storage_bool(this.0, this.1, $key.as_ptr(), $key.len());
        error!(error);
        value
    }};
    (boolean :: $address:expr => $key:expr) => {{
        let (error, value) = bindings::v0::get_storage_bool(
            $address.as_ptr(),
            $address.len(),
            $key.as_ptr(),
            $key.len(),
        );
        error!(error);
        value
    }};
    (binary :: $key:expr) => {{
        let this = internal_data!(this);
        let (error, ptr, len) =
            bindings::v0::get_storage_binary(this.0, this.1, $key.as_ptr(), $key.len());
        error!(error);
        core::slice::from_raw_parts(ptr, len)
    }};
    (binary :: $address:expr => $key:expr) => {{
        let (error, ptr, len) = bindings::v0::get_storage_binary(
            $address.as_ptr(),
            $address.len(),
            $key.as_ptr(),
            $key.len(),
        );
        error!(error);
        core::slice::from_raw_parts(ptr, len)
    }};
    (string :: $key:expr) => {{
        let this = internal_data!(this);
        let (error, ptr, len) =
            bindings::v0::get_storage_string(this.0, this.1, $key.as_ptr(), $key.len());
        error!(error);
        let bytes = core::slice::from_raw_parts(ptr, len);
        core::str::from_utf8_unchecked(bytes)
    }};
    (string :: $address:expr => $key:expr) => {{
        let (error, ptr, len) = bindings::v0::get_storage_string(
            $address.as_ptr(),
            $address.len(),
            $key.as_ptr(),
            $key.len(),
        );
        error!(error);
        let bytes = core::slice::from_raw_parts(ptr, len);
        core::str::from_utf8_unchecked(bytes)
    }};
}

/// Set storage
///
/// # Usage
/// ```
/// use we_cdk::*;
///
/// #[action]
/// fn _constructor() {
///     set_storage!(integer :: "integer_key" => 42);
///     set_storage!(boolean :: "boolean_key" => true);
///     set_storage!(binary :: "binary_key" => &[0, 1]);
///     set_storage!(string :: "string_key" => "test");
/// }
/// ```
#[macro_export]
macro_rules! set_storage {
    (integer :: $key:expr => $value:expr) => {{
        let error = bindings::v0::set_storage_int($key.as_ptr(), $key.len(), $value);
        error!(error);
    }};
    (boolean :: $key:expr => $value:expr) => {
        let error = bindings::v0::set_storage_bool($key.as_ptr(), $key.len(), $value);
        error!(error);
    };
    (binary :: $key:expr => $value:expr) => {
        let error = bindings::v0::set_storage_binary(
            $key.as_ptr(),
            $key.len(),
            $value.as_ptr(),
            $value.len(),
        );
        error!(error);
    };
    (string :: $key:expr => $value:expr) => {
        let error = bindings::v0::set_storage_string(
            $key.as_ptr(),
            $key.len(),
            $value.as_ptr(),
            $value.len(),
        );
        error!(error);
    };
}
