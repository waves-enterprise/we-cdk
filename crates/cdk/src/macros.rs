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
///     require!(balance > 0);
/// }
/// ```
#[macro_export]
macro_rules! require {
    ($condition:expr) => {
        if !($condition) {
            return 300;
        }
    };
}
