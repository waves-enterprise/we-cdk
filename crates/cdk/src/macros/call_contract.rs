/// Call contract
///
/// # Usage
/// ```
/// use we_cdk::*;
///
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
///
/// The contract can also be called with raw data of type Binary
///
/// ```
/// use we_cdk::*;
///
/// #[action]
/// fn call_with_binary_params(func_name: String, params: Binary) {
///     let contract = base58!("4WVhw3QdiinpE5QXDG7QfqLiLanM7ewBw4ChX4qyGjs2");
///
///     call_contract! {
///         (contract)::call(func_name, params)
///     };
/// }
/// ```
#[macro_export]
macro_rules! call_contract {
    ($interface:ident ( $contract_id:expr ) :: $func_name:ident ( $($func_args:expr),* ) $(:: payments ( $($payment_args:expr),+ ))?) => {
        $(
            $(
                let error = bindings::v0::call_payment($payment_args.0.as_ptr(), $payment_args.0.len(), $payment_args.1);
                error!(error);
            )+
        )?
        let error = $interface::$func_name($contract_id, $($func_args),* );
        error!(error);
    };
    ( ($contract_id:expr) :: call ($func_name:expr, $params:expr ) $(:: payments ( $($payment_args:expr),+ ))?) => {
        $(
            $(
                let error = bindings::v0::call_payment($payment_args.0.as_ptr(), $payment_args.0.len(), $payment_args.1);
                error!(error);
            )+
        )?
        let error = bindings::v0::call_contract_params(
            $contract_id.as_ptr(),
            $contract_id.len(),
            $func_name.as_ptr(),
            $func_name.len(),
            $params.as_ptr(),
            $params.len(),
        );
        error!(error);
    };
}
