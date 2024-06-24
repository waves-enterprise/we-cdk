mod asset;
mod block;
mod call_contract;
mod converts;
mod crypto;
mod lease;
mod memory;
mod storage;
mod tx;
mod utils;

#[doc(hidden)]
#[macro_export]
macro_rules! internal_data {
    (this) => {{
        (core::ptr::null(), 0)
    }};
    (system_token) => {{
        (core::ptr::null(), 0)
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! error {
    ($error:expr) => {
        if $error != 0 {
            return $error;
        }
    };
}

/// Verify inputs and conditions
///
/// # Result
/// If the condition was not satisfied,
/// the execution will be stopped with error code 300 (RuntimeError::Exception)
///
/// # Usage
/// ```
/// use we_cdk::*;
///
/// #[action]
/// fn _constructor() {
///     let balance = get_balance!(this);
///     require!(balance > 42);
/// }
///
/// #[action]
/// fn require_with_message() {
///     let balance = get_balance!(this);
///     require!(balance > 1337, "Balance is less than 1337!");
/// }
/// ```
#[macro_export]
macro_rules! require {
    ($condition:expr) => {
        if !($condition) {
            return 300;
        }
    };
    ($condition:expr, $message:tt) => {
        if !($condition) {
            let error = wevm::v0::bindings::require($message.as_ptr(), $message.len());
            if error != 0 {
                return error;
            } else {
                return 300;
            }
        }
    }
}
