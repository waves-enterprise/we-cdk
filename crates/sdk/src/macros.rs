#[macro_export]
macro_rules! internal_data {
    (this) => {{
        (core::ptr::null(), 0)
    }};
    (system_token) => {{
        (core::ptr::null(), 0)
    }};
}

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
            return 300;
        }
    };
}

/// Base58 string to bytes conversion
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
    ($value:expr) => {{
        let (error, ptr, len) = base_58($value.as_ptr(), $value.len());
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
/// use we_contract_sdk::*;
///
/// #[action]
/// fn _constructor() {
///     let address = base58!("3NzkzibVRkKUzaRzjUxndpTPvoBzQ3iLng3");
///     let address_string = to_base58_string!(address);
/// }
/// ```
#[macro_export]
macro_rules! to_base58_string {
    ($value:expr) => {{
        let (error, ptr, len) = to_base_58_string($value.as_ptr(), $value.len());
        error!(error);
        core::slice::from_raw_parts(ptr, len)
    }};
}

/// Call contract
///
/// # Usage
/// ```
/// use we_contract_sdk::*;
/// #[interface]
/// trait i_contract {
///     fn method(integer: Integer, boolean: Boolean, binary: Binary, string: String);
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
///
///     call_contract! {
///         i_contract(contract)::method(integer, boolean, binary, string)
///     };
/// }
/// ```
#[macro_export]
macro_rules! call_contract {
    ($interface:ident ( $contract_id:expr ) :: $func_name:ident ( $($func_args:expr),* ) $(:: payments ( $($payment_args:expr),+ ))?) => {
        $(
            $(
                let error = call_payment($payment_args.0.as_ptr(), $payment_args.0.len(), $payment_args.1);
                error!(error);
            )+
        )?
        let error = $interface::$func_name($contract_id, $($func_args),* );
        error!(error);
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
///     let integer_value: Integer = get_storage!(integer "integer_key");
///     let boolean_value: Boolean = get_storage!(boolean "boolean_key");
///     let binary_value: Binary = get_storage!(binary "binary_key");
///     let string_value: String = get_storage!(string "string_key");
///
///     let address_int_value = get_storage!(address => integer "integer_key");
/// }
/// ```
#[macro_export]
macro_rules! get_storage {
    (integer $key:expr) => {{
        let this = internal_data!(this);
        let (error, value) = get_storage_int(this.0, this.1, $key.as_ptr(), $key.len());
        error!(error);
        value
    }};
    ($address:expr => integer $key:expr) => {{
        let (error, value) =
            get_storage_int($address.as_ptr(), $address.len(), $key.as_ptr(), $key.len());
        error!(error);
        value
    }};
    (boolean $key:expr) => {{
        let this = internal_data!(this);
        let (error, value) = get_storage_bool(this.0, this.1, $key.as_ptr(), $key.len());
        error!(error);
        value
    }};
    ($address:expr => boolean $key:expr) => {{
        let (error, value) =
            get_storage_bool($address.as_ptr(), $address.len(), $key.as_ptr(), $key.len());
        error!(error);
        value
    }};
    (binary $key:expr) => {{
        let this = internal_data!(this);
        let (error, ptr, len) = get_storage_binary(this.0, this.1, $key.as_ptr(), $key.len());
        error!(error);
        core::slice::from_raw_parts(ptr, len)
    }};
    ($address:expr => binary $key:expr) => {{
        let (error, ptr, len) =
            get_storage_binary($address.as_ptr(), $address.len(), $key.as_ptr(), $key.len());
        error!(error);
        core::slice::from_raw_parts(ptr, len)
    }};
    (string $key:expr) => {{
        let this = internal_data!(this);
        let (error, ptr, len) = get_storage_string(this.0, this.1, $key.as_ptr(), $key.len());
        error!(error);
        let bytes = core::slice::from_raw_parts(ptr, len);
        core::str::from_utf8_unchecked(bytes)
    }};
    ($address:expr => string $key:expr) => {{
        let (error, ptr, len) =
            get_storage_string($address.as_ptr(), $address.len(), $key.as_ptr(), $key.len());
        error!(error);
        let bytes = core::slice::from_raw_parts(ptr, len);
        core::str::from_utf8_unchecked(bytes)
    }};
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
    (integer => ( $key:expr, $value:expr )) => {{
        let error = set_storage_int($key.as_ptr(), $key.len(), $value);
        error!(error);
    }};
    (boolean => ( $key:expr, $value:expr )) => {
        let error = set_storage_bool($key.as_ptr(), $key.len(), $value);
        error!(error);
    };
    (binary => ( $key:expr, $value:expr )) => {
        let error = set_storage_binary($key.as_ptr(), $key.len(), $value.as_ptr(), $value.len());
        error!(error);
    };
    (string => ( $key:expr, $value:expr )) => {
        let error = set_storage_string($key.as_ptr(), $key.len(), $value.as_ptr(), $value.len());
        error!(error);
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
    (this) => {{
        let system_token = internal_data!(system_token);
        let this = internal_data!(this);
        let (error, balance) = get_balance(system_token.0, system_token.1, this.0, this.1);
        error!(error);
        balance
    }};
    (this, asset => $asset_id:expr) => {{
        let this = internal_data!(this);
        let (error, balance) = get_balance($asset_id.as_ptr(), $asset_id.len(), this.0, this.1);
        error!(error);
        balance
    }};
    (address => $address:expr) => {{
        let system_token = internal_data!(system_token);
        let (error, balance) = get_balance(
            system_token.0,
            system_token.1,
            $address.as_ptr(),
            $address.len(),
        );
        error!(error);
        balance
    }};
    (address => $address:expr, asset => $asset_id:expr) => {{
        let (error, balance) = get_balance(
            $asset_id.as_ptr(),
            $asset_id.len(),
            $address.as_ptr(),
            $address.len(),
        );
        error!(error);
        balance
    }};
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
        let system_token = internal_data!(system_token);
        let error = transfer(
            system_token.0,
            system_token.1,
            $recipient.as_ptr(),
            $recipient.len(),
            $amount,
        );
        error!(error);
    };
    ($asset_id:expr, $recipient:expr, $amount:expr) => {
        let error = transfer(
            $asset_id.as_ptr(),
            $asset_id.len(),
            $recipient.as_ptr(),
            $recipient.len(),
            $amount,
        );
        error!(error);
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
    ($name:expr, $description:expr, $quantity:expr, $decimals:expr, $is_reissuable:expr) => {{
        let (error, ptr, len) = issue(
            $name.as_ptr(),
            $name.len(),
            $description.as_ptr(),
            $description.len(),
            $quantity,
            $decimals,
            $is_reissuable,
        );
        error!(error);
        core::slice::from_raw_parts(ptr, len)
    }};
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
        let error = burn($asset_id.as_ptr(), $asset_id.len(), $amount);
        error!(error);
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
        let error = reissue($asset_id.as_ptr(), $asset_id.len(), $amount, $is_reissuable);
        error!(error);
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
///     let lease_id_first = lease!(address => recipient, amount);
///     let lease_id_second = lease!(alias => "miner", amount);
/// }
/// ```
#[macro_export]
macro_rules! lease {
    (address => $recipient:expr, $amount:expr) => {{
        let (error, ptr, len) = lease_address($recipient.as_ptr(), $recipient.len(), $amount);
        error!(error);
        core::slice::from_raw_parts(ptr, len)
    }};
    (alias => $recipient:expr, $amount:expr) => {{
        let (error, ptr, len) = lease_alias($recipient.as_ptr(), $recipient.len(), $amount);
        error!(error);
        core::slice::from_raw_parts(ptr, len)
    }};
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
        let error = cancel_lease($lease_id.as_ptr(), $lease_id.len());
        error!(error);
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
    () => {{
        let (error, value) = get_block_timestamp();
        error!(error);
        value
    }};
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
    () => {{
        let (error, value) = get_block_height();
        error!(error);
        value
    }};
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
    () => {{
        let (error, ptr, len) = get_tx_sender();
        error!(error);
        core::slice::from_raw_parts(ptr, len)
    }};
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
    () => {{
        let (error, value) = get_tx_payments();
        error!(error);
        value
    }};
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
///     let payment: Payment = get_tx_payment!(0);
///     let asset_id = payment.0;
///     let amount = payment.1;
/// }
/// ```
#[macro_export]
macro_rules! get_tx_payment {
    ($number:expr) => {{
        let asset_id = {
            let (error, ptr, len) = get_tx_payment_asset_id($number);
            error!(error);
            core::slice::from_raw_parts(ptr, len)
        };

        let amount = {
            let (error, value) = get_tx_payment_amount($number);
            error!(error);
            value
        };
        (asset_id, amount)
    }};
}
