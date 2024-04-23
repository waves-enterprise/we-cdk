/// Get the balance of tokens
///
/// # Result
/// The result of execution is `i64` balance
///
/// # Usage
/// ```
/// use we_cdk::*;
///
/// #[action]
/// fn _constructor() {
///     let token: Binary = base58!("DnK5Xfi2wXUJx9BjK9X6ZpFdTLdq2GtWH9pWrcxcmrhB");
///     let another_address: Binary = base58!("3NzkzibVRkKUzaRzjUxndpTPvoBzQ3iLng3");
///     let another_alias: String = "alias";
///     let another_contract: Binary = base58!("4WVhw3QdiinpE5QXDG7QfqLiLanM7ewBw4ChX4qyGjs2");
///
///     let contract_balance: Integer = get_balance!(this);
///     let contract_asset_balance: Integer = get_balance!(this, asset => token);
///     let address_balance: Integer = get_balance!(address => another_address);
///     let alias_balance: Integer = get_balance!(alias => another_alias);
///     let contract_balance: Integer = get_balance!(contract => another_contract);
///     let address_asset_balance: Integer = get_balance!(address => another_address, asset => token);
/// }
/// ```
#[macro_export]
macro_rules! get_balance {
    // For use within a macro
    (@inner, $holder:expr, $type:expr, $version:expr) => {{
        let system_token = internal_data!(system_token);
        let (error, balance) = wevm::v1::bindings::get_balance(
            system_token.0,
            system_token.1,
            $holder.as_ptr(),
            $holder.len(),
            $type,
            $version,
        );
        error!(error);
        balance
    }};
    // For use within a macro
    (@inner, $holder:expr, $asset_id:expr, $type:expr, $version:expr) => {{
        let (error, balance) = wevm::v1::bindings::get_balance(
            $asset_id.as_ptr(),
            $asset_id.len(),
            $holder.as_ptr(),
            $holder.len(),
            $type,
            $version,
        );
        error!(error);
        balance
    }};
    (this) => {{
        let system_token = internal_data!(system_token);
        let this = internal_data!(this);
        let (error, balance) =
            wevm::v0::bindings::get_balance(system_token.0, system_token.1, this.0, this.1);
        error!(error);
        balance
    }};
    (this, asset => $asset_id:expr) => {{
        let this = internal_data!(this);
        let (error, balance) =
            wevm::v0::bindings::get_balance($asset_id.as_ptr(), $asset_id.len(), this.0, this.1);
        error!(error);
        balance
    }};
    (address => $address:expr) => {
        get_balance!(@inner, $address, 0, 1)
    };
    (address => $address:expr, asset => $asset_id:expr) => {
        get_balance!(@inner, $address, $asset_id, 0, 1)
    };
    (alias => $address:expr) => {
        get_balance!(@inner, $address, 0, 2)
    };
    (alias => $address:expr, asset => $asset_id:expr) => {
        get_balance!($address, $asset_id, 0, 2)
    };
    (contract => $address:expr) => {
        get_balance!(@inner, $address, 1, 1)
    };
    (contract => $address:expr, asset => $asset_id:expr) => {
        get_balance!($address, $asset_id, 1, 1)
    };
}

/// Tokens transfer
///
/// # Usage
/// ```
/// use we_cdk::*;
///
/// #[action]
/// fn _constructor() {
///     let asset_id: Binary = base58!("DnK5Xfi2wXUJx9BjK9X6ZpFdTLdq2GtWH9pWrcxcmrhB");
///     let recipient: Binary = base58!("3NzkzibVRkKUzaRzjUxndpTPvoBzQ3iLng3");
///     let amount: Integer = 100;
///     transfer!(address => recipient, amount);
///     transfer!(asset => asset_id, address => recipient, amount);
/// }
/// ```
#[macro_export]
macro_rules! transfer {
    // For use within a macro
    (@inner, $recipient:expr, $amount:expr, $type:expr, $version:expr) => {
        let system_token = internal_data!(system_token);
        let error = wevm::v1::bindings::transfer(
            system_token.0,
            system_token.1,
            $recipient.as_ptr(),
            $recipient.len(),
            $type,
            $version,
            $amount,
        );
        error!(error);
    };
    // For use within a macro
    (@inner, $asset_id:expr, $recipient:expr, $amount:expr, $type:expr, $version:expr) => {
        let error = wevm::v1::bindings::transfer(
            $asset_id.as_ptr(),
            $asset_id.len(),
            $recipient.as_ptr(),
            $recipient.len(),
            $type,
            $version,
            $amount,
        );
        error!(error);
    };
    (address => $recipient:expr, $amount:expr) => {
        transfer!(@inner, $recipient, $amount, 0, 1);
    };
    (alias => $recipient:expr, $amount:expr) => {
        transfer!(@inner, $recipient, $amount, 0, 2);
    };
    (contract => $recipient:expr, $amount:expr) => {
        transfer!(@inner, $recipient, $amount, 1, 1);
    };
    (asset => $asset_id:expr, address => $recipient:expr, $amount:expr) => {
        transfer!(@inner, $asset_id, $recipient, $amount, 0, 1);
    };
    (asset => $asset_id:expr, alias => $recipient:expr, $amount:expr) => {
        transfer!(@inner, $asset_id, $recipient, $amount, 0, 2);
    };
    (asset => $asset_id:expr, contract => $recipient:expr, $amount:expr) => {
        transfer!(@inner, $asset_id, $recipient, $amount, 1, 1);
    };
}

/// Issue the asset
///
/// # Result
/// The result of execution is `&[u8]` asset_id
///
/// # Usage
/// ```
/// use we_cdk::*;
///
/// #[action]
/// fn _constructor() {
///     let name: String = "TEST";
///     let description: String = "Test asset";
///     let quantity: Integer = 1_000_000;
///     let decimals: Integer = 8;
///     let is_reissuable: Boolean = true;
///     let asset_id: Binary = issue!(name, description, quantity, decimals, is_reissuable);
/// }
/// ```
#[macro_export]
macro_rules! issue {
    ($name:expr, $description:expr, $quantity:expr, $decimals:expr, $is_reissuable:expr) => {{
        let (error, ptr, len) = wevm::v1::bindings::issue(
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
/// use we_cdk::*;
///
/// #[action]
/// fn _constructor() {
///     let asset_id: Binary = base58!("DnK5Xfi2wXUJx9BjK9X6ZpFdTLdq2GtWH9pWrcxcmrhB");
///     let amount: Integer = 100;
///     burn!(asset_id, amount);
/// }
/// ```
#[macro_export]
macro_rules! burn {
    ($asset_id:expr, $amount:expr) => {
        let error = wevm::v0::bindings::burn($asset_id.as_ptr(), $asset_id.len(), $amount);
        error!(error);
    };
}

/// Reissue the asset
///
/// # Usage
/// ```
/// use we_cdk::*;
///
/// #[action]
/// fn _constructor() {
///     let asset_id: Binary = base58!("DnK5Xfi2wXUJx9BjK9X6ZpFdTLdq2GtWH9pWrcxcmrhB");
///     let amount: Integer = 100;
///     let is_reissuable: Boolean = true;
///     reissue!(asset_id, amount, is_reissuable);
/// }
/// ```
#[macro_export]
macro_rules! reissue {
    ($asset_id:expr, $amount:expr, $is_reissuable:expr) => {
        let error = wevm::v0::bindings::reissue(
            $asset_id.as_ptr(),
            $asset_id.len(),
            $amount,
            $is_reissuable,
        );
        error!(error);
    };
}
