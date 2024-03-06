/// Get block fields
///
/// # Usage
/// ```
/// use we_cdk::*;
///
/// #[action]
/// fn _constructor() {
///     let block_timestamp: Integer = block!(timestamp);
///     let block_height: Integer = block!(height);
/// }
/// ```
#[macro_export]
macro_rules! block {
    (timestamp) => {{
        let (error, value) = bindings::v0::get_block_timestamp();
        error!(error);
        value
    }};
    (height) => {{
        let (error, value) = bindings::v0::get_block_height();
        error!(error);
        value
    }};
}
