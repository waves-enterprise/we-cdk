mod macros;

pub use we_contract_codegen::*;
pub use we_contract_proc_macro::*;

pub type Integer = i64;
pub type Boolean = bool;
pub type Binary<'a> = &'a [u8];
pub type String<'a> = &'a str;
pub type Payment<'a> = (&'a [u8], i64);

pub const SYSTEM_TOKEN: &[u8] = &[0u8; 0];
pub const THIS: &[u8] = &[0u8; 0];
