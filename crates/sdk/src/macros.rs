#[macro_export]
macro_rules! error_handling {
    ($func:expr) => {
        let result = $func;
        if result != 0 {
            return result;
        }
    };
}

#[macro_export]
macro_rules! error_handling_tuple {
    ($func:expr) => {{
        let result = $func;
        if result.0 != 0 {
            return result.0;
        }
        result.1
    }};
}

/// Verify inputs and conditions
///
/// # Usage
/// ```
/// use we_contract_sdk::*;
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
            return -1;
        }
    };
}

/// Base58 conversion
///
/// # Result
/// The result of execution is `&[u8]` bytes
///
/// # Usage
/// ```
/// use we_contract_sdk::*;
///
/// #[action]
/// fn _constructor() {
///     let address = base58!("3NzkzibVRkKUzaRzjUxndpTPvoBzQ3iLng3");
/// }
/// ```
#[macro_export]
macro_rules! base58 {
    ($value:expr) => {
        error_handling_tuple!(base_58($value))
    };
}

/// Call contract
///
/// # Usage
/// ```
/// use we_contract_sdk::*;
/// #[interface]
/// trait i_contract {
///     fn method(integer: Integer, boolean: Boolean, binary: Binary, string: String, payment: Payment);
/// }
///
/// #[action]
/// fn _constructor() {
///     let contract = base58!("4WVhw3QdiinpE5QXDG7QfqLiLanM7ewBw4ChX4qyGjs2");
///     let asset = base58!("DnK5Xfi2wXUJx9BjK9X6ZpFdTLdq2GtWH9pWrcxcmrhB");
///
///     let integer: Integer = 42;
///     let boolean: Boolean = true;
///     let binary: Binary = &[0, 1];
///     let string: String = "test";
///     let payment: Payment = (asset, 2400000000);
///
///     call_contract! {
///         i_contract(contract) => method(integer, boolean, binary, string, payment)
///     };
/// }
/// ```
#[macro_export]
macro_rules! call_contract {
    ($interface:ident ( $contract_id:expr ) => $func_name:ident ( $($args:expr),* )) => {
        error_handling!($interface::$func_name($contract_id, $($args),* ));
    };
}

/// Get storage
///
/// # Usage
/// ```
/// use we_contract_sdk::*;
///
/// #[action]
/// fn _constructor() {
///     let address = base58!("3NzkzibVRkKUzaRzjUxndpTPvoBzQ3iLng3");
///
///     let integer_value: i64 = get_storage!(integer "integer_key");
///     let boolean_value: bool = get_storage!(boolean "boolean_key");
///     let binary_value: &[u8] = get_storage!(binary "binary_key");
///     let string_value: &str = get_storage!(string "string_key");
///
///     let address_int_value = get_storage!(address => integer "integer_key");
/// }
/// ```
#[macro_export]
macro_rules! get_storage {
    (integer $key:expr) => {
        error_handling_tuple!(get_storage_int(&[], $key))
    };
    ($address:expr => integer $key:expr) => {
        error_handling_tuple!(get_storage_int($address, $key))
    };
    (boolean $key:expr) => {
        error_handling_tuple!(get_storage_bool(&[], $key))
    };
    ($address:expr => boolean $key:expr) => {
        error_handling_tuple!(get_storage_bool($address, $key))
    };
    (binary $key:expr) => {
        error_handling_tuple!(get_storage_binary(&[], $key))
    };
    ($address:expr => binary $key:expr) => {
        error_handling_tuple!(get_storage_binary($address, $key))
    };
    (string $key:expr) => {
        error_handling_tuple!(get_storage_string(&[], $key))
    };
    ($address:expr => string $key:expr) => {
        error_handling_tuple!(get_storage_string($address, $key))
    };
}

/// Set storage
///
/// # Usage
/// ```
/// use we_contract_sdk::*;
///
/// #[action]
/// fn _constructor() {
///     set_storage!(integer => ("integer_key", 42));
///     set_storage!(boolean => ("boolean_key", true));
///     set_storage!(binary => ("binary_key", &[0, 1]));
///     set_storage!(string => ("string_key", "test"));
/// }
/// ```
#[macro_export]
macro_rules! set_storage {
    (integer => ( $key:expr, $value:expr )) => {
        error_handling!(set_storage_int($key, $value));
    };
    (boolean => ( $key:expr, $value:expr )) => {
        error_handling!(set_storage_bool($key, $value));
    };
    (binary => ( $key:expr, $value:expr )) => {
        error_handling!(set_storage_binary($key, $value));
    };
    (string => ( $key:expr, $value:expr )) => {
        error_handling!(set_storage_string($key, $value));
    };
}

/// Get the balance of tokens
///
/// # Result
/// The result of execution is `i64` balance
///
/// # Usage
/// ```
/// use we_contract_sdk::*;
///
/// #[action]
/// fn _constructor() {
///     let token = base58!("DnK5Xfi2wXUJx9BjK9X6ZpFdTLdq2GtWH9pWrcxcmrhB");
///     let another_address = base58!("3NzkzibVRkKUzaRzjUxndpTPvoBzQ3iLng3");
///
///     let contract_balance = get_balance!(this);
///     let contract_asset_balance = get_balance!(this, asset => token);
///     let address_balance = get_balance!(address => another_address);
///     let address_asset_balance = get_balance!(address => another_address, asset => token);
/// }
/// ```
#[macro_export]
macro_rules! get_balance {
    (this) => {
        error_handling_tuple!(get_balance(&[], &[]))
    };
    (this, asset => $asset_id:expr) => {
        error_handling_tuple!(get_balance($asset_id, &[]))
    };
    (address => $address:expr) => {
        error_handling_tuple!(get_balance(&[], $address))
    };
    (address => $address:expr, asset => $asset_id:expr) => {
        error_handling_tuple!(get_balance($asset_id, $address))
    };
}

/// Tokens transfer
///
/// # Usage
/// ```
/// use we_contract_sdk::*;
///
/// #[action]
/// fn _constructor() {
///     let asset_id = base58!("DnK5Xfi2wXUJx9BjK9X6ZpFdTLdq2GtWH9pWrcxcmrhB");
///     let recipient = base58!("3NzkzibVRkKUzaRzjUxndpTPvoBzQ3iLng3");
///     let amount = 100;
///     transfer!(recipient, amount);
///     transfer!(asset_id, recipient, amount);
/// }
/// ```
#[macro_export]
macro_rules! transfer {
    ($recipient:expr, $amount:expr) => {
        error_handling!(transfer(&[], $recipient, $amount))
    };
    ($asset_id:expr, $recipient:expr, $amount:expr) => {
        error_handling!(transfer($asset_id, $recipient, $amount))
    };
}

/// Issue the asset
///
/// # Result
/// The result of execution is `&[u8]` asset_id
///
/// # Usage
/// ```
/// use we_contract_sdk::*;
///
/// #[action]
/// fn _constructor() {
///     let name = "TEST";
///     let description = "Test asset";
///     let quantity = 1_000_000;
///     let decimals = 8;
///     let is_reissuable = true;
///     let asset_id = issue!(name, description, quantity, decimals, is_reissuable);
/// }
/// ```
#[macro_export]
macro_rules! issue {
    ($name:expr, $description:expr, $quantity:expr, $decimals:expr, $is_reissuable:expr) => {
        error_handling_tuple!(issue(
            $name,
            $description,
            $quantity,
            $decimals,
            $is_reissuable
        ))
    };
}

/// Burn the asset
///
/// # Usage
/// ```
/// use we_contract_sdk::*;
///
/// #[action]
/// fn _constructor() {
///     let asset_id = base58!("DnK5Xfi2wXUJx9BjK9X6ZpFdTLdq2GtWH9pWrcxcmrhB");
///     let amount = 100;
///     burn!(asset_id, amount);
/// }
/// ```
#[macro_export]
macro_rules! burn {
    ($asset_id:expr, $amount:expr) => {
        error_handling!(burn($asset_id, $amount))
    };
}

/// Reissue the asset
///
/// # Usage
/// ```
/// use we_contract_sdk::*;
///
/// #[action]
/// fn _constructor() {
///     let asset_id = base58!("DnK5Xfi2wXUJx9BjK9X6ZpFdTLdq2GtWH9pWrcxcmrhB");
///     let amount = 100;
///     let is_reissuable = true;
///     reissue!(asset_id, amount, is_reissuable);
/// }
/// ```
#[macro_export]
macro_rules! reissue {
    ($asset_id:expr, $amount:expr, $is_reissuable:expr) => {
        error_handling!(reissue($asset_id, $amount, $is_reissuable))
    };
}

/// Leasing of funds
///
/// # Result
/// The result of execution is `&[u8]` lease_id
///
/// # Usage
/// ```
/// use we_contract_sdk::*;
///
/// #[action]
/// fn _constructor() {
///     let recipient = base58!("3NzkzibVRkKUzaRzjUxndpTPvoBzQ3iLng3");
///     let amount = 100;
///     let lease_id = lease!(recipient, amount);
/// }
/// ```
#[macro_export]
macro_rules! lease {
    ($recipient:expr, $amount:expr) => {
        error_handling_tuple!(lease($recipient, $amount));
    };
}

/// Cancel the lease
///
/// # Usage
/// ```
/// use we_contract_sdk::*;
///
/// #[action]
/// fn _constructor() {
///     let lease_id = base58!("6Tn7ir9MycHW6Gq2F2dGok2stokSwXJadPh4hW8eZ8Sp");
///     cancel_lease!(lease_id);
/// }
/// ```
#[macro_export]
macro_rules! cancel_lease {
    ($lease_id:expr) => {
        error_handling!(cancel_lease($lease_id))
    };
}

/// Get block timestamp
///
/// # Result
/// The result of execution is `i64` timestamp
///
/// # Usage
/// ```
/// use we_contract_sdk::*;
///
/// #[action]
/// fn _constructor() {
///     let block_timestamp = get_block_timestamp!();
/// }
/// ```
#[macro_export]
macro_rules! get_block_timestamp {
    () => {
        error_handling_tuple!(get_block_timestamp())
    };
}

/// Get block height
///
/// # Result
/// The result of execution is `i64` height
///
/// # Usage
/// ```
/// use we_contract_sdk::*;
///
/// #[action]
/// fn _constructor() {
///     let block_height = get_block_height!();
/// }
/// ```
#[macro_export]
macro_rules! get_block_height {
    () => {
        error_handling_tuple!(get_block_height())
    };
}

/// Get the address of the calling contract
///
/// # Result
/// The result of execution is `&[u8]` address
///
/// # Usage
/// ```
/// use we_contract_sdk::*;
///
/// #[action]
/// fn _constructor() {
///     let tx_sender = get_tx_sender!();
/// }
/// ```
#[macro_export]
macro_rules! get_tx_sender {
    () => {
        error_handling_tuple!(get_tx_sender())
    };
}

/// Get the number of attached payments in the transaction
///
/// # Result
/// The result of execution is `i32` number
///
/// # Usage
/// ```
/// use we_contract_sdk::*;
///
/// #[action]
/// fn _constructor() {
///     let number_payments = get_tx_payments!();
/// }
/// ```
#[macro_export]
macro_rules! get_tx_payments {
    () => {
        error_handling_tuple!(get_tx_payments())
    };
}

/// Get the `asset_id` or `amount` of the attached payment in the transaction
///
/// # Param
/// The function expects the order number of the attached payment as input
///
/// # Result
/// The result of execution is `&[u8]` for `asset_id` and `i64` for `amount`
///
/// # Usage
/// ```
/// use we_contract_sdk::*;
///
/// #[action]
/// fn _constructor() {
///     let payment_asset = get_tx_payment!(asset_id => 1);
///     let payment_amount = get_tx_payment!(amount => 1);
/// }
/// ```
#[macro_export]
macro_rules! get_tx_payment {
    (asset_id => $number:expr) => {
        error_handling_tuple!(get_tx_payment_asset_id($number))
    };
    (amount => $number:expr) => {
        error_handling_tuple!(get_tx_payment_amount($number))
    };
}
