//! `````no_test
#![doc = include_str!("../README.md")]
//! `````
pub mod macros;

pub use we_contract_proc_macro::*;
pub use wevm;

/// Integer is an integer data type.
pub type Integer = i64;
/// Boolean is a boolean data type.
pub type Boolean = bool;
/// Binary is a data type for byte array.
pub type Binary<'a> = &'a [u8];
/// String is a string data type.
/// Strings are UTF-8 encoded.
pub type String<'a> = &'a str;

/// Payment is a payment that can be attached when calling the function of another contract.
pub type Payment<'a> = (&'a [u8], i64);

#[doc(hidden)]
pub const SYSTEM_TOKEN: &[u8] = &[0u8; 0];
#[doc(hidden)]
pub const THIS: &[u8] = &[0u8; 0];
