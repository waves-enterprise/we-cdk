/// Get the number of attached payments in the transaction
///
/// # Result
/// The result of execution is `i32` number
///
/// # Usage
/// ```
/// use we_cdk::*;
///
/// #[action]
/// fn _constructor() {
///     let number_payments: Integer = get_tx_payments!();
/// }
/// ```
#[macro_export]
macro_rules! get_tx_payments {
    () => {{
        let (error, value) = wevm::v1::bindings::get_payments();
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
/// use we_cdk::*;
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
            let (error, ptr, len) = wevm::v1::bindings::get_payment_asset_id($number);
            error!(error);
            core::slice::from_raw_parts(ptr, len)
        };

        let amount = {
            let (error, value) = wevm::v1::bindings::get_payment_amount($number);
            error!(error);
            value
        };
        (asset_id, amount)
    }};
}

/// Get tx fields
///
/// # Usage
/// ```
/// use we_cdk::*;
///
/// #[action]
/// fn _constructor() {
///     let tx_id: Binary = tx!(tx_id);
///     let tx_sender: Binary = tx!(sender);
/// }
/// ```
#[macro_export]
macro_rules! tx {
    (tx_id) => {{
        let field = "txId";
        let (error, ptr, len) = wevm::v1::bindings::tx(field.as_ptr(), field.len());
        error!(error);
        core::slice::from_raw_parts(ptr, len)
    }};
    (sender) => {{
        let field = "sender";
        let (error, ptr, len) = wevm::v1::bindings::tx(field.as_ptr(), field.len());
        error!(error);
        core::slice::from_raw_parts(ptr, len)
    }};
}
