mod bindings;
pub mod generator;

pub use bindings::*;

pub type Integer = i64;
pub type Boolean = bool;
pub type Binary<'a> = &'a [u8];
pub type String<'a> = &'a str;
pub type Payment<'a> = (&'a [u8], i64);

pub const SYSTEM_TOKEN: &[u8] = &[];
pub const THIS: &[u8] = &[];
