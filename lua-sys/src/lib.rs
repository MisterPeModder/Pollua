//! lua-sys - Raw ffi bindings for Lua 5.3

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]
#![cfg_attr(rust_nightly, allow(clippy::missing_safety_doc))]

extern crate libc;
#[cfg(feature = "va-list")]
extern crate va_list;

mod lcore;
mod ldebug;

pub use lcore::*;
pub use ldebug::*;
