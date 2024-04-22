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
        let field = "timestamp";
        let (error, ptr, len) = wevm::v1::bindings::block(field.as_ptr(), field.len());
        error!(error);
        let (error, ptr, len) = wevm::v0::bindings::to_le_bytes(ptr, len);
        error!(error);
        *(ptr as *const i64)
    }};
    (height) => {{
        let field = "height";
        let (error, ptr, len) = wevm::v1::bindings::block(field.as_ptr(), field.len());
        error!(error);
        let (error, ptr, len) = wevm::v0::bindings::to_le_bytes(ptr, len);
        error!(error);
        *(ptr as *const i64)
    }};
}
