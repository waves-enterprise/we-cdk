mod bindings;
pub mod generator;

pub use bindings::*;

pub type Integer = i64;
pub type Boolean = bool;
pub type Binary<'a> = &'a [u8];
pub type String<'a> = &'a str;
pub type Payment<'a> = (&'a [u8], i64);

pub const SYSTEM_TOKEN: (*const u8, usize) = (core::ptr::null(), 0);
pub const THIS: (*const u8, usize) = (core::ptr::null(), 0);
