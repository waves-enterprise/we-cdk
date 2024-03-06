/// Leasing of funds
///
/// # Result
/// The result of execution is `&[u8]` lease_id
///
/// # Usage
/// ```
/// use we_cdk::*;
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
        let (error, ptr, len) =
            bindings::v0::lease_address($recipient.as_ptr(), $recipient.len(), $amount);
        error!(error);
        core::slice::from_raw_parts(ptr, len)
    }};
    (alias => $recipient:expr, $amount:expr) => {{
        let (error, ptr, len) =
            bindings::v0::lease_alias($recipient.as_ptr(), $recipient.len(), $amount);
        error!(error);
        core::slice::from_raw_parts(ptr, len)
    }};
}

/// Cancel the lease
///
/// # Usage
/// ```
/// use we_cdk::*;
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
        let error = bindings::v0::cancel_lease($lease_id.as_ptr(), $lease_id.len());
        error!(error);
    };
}
