//! lua-sys - Raw ffi bindings for Lua 5.3

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]
#![cfg_attr(rust_nightly, allow(clippy::missing_safety_doc))]

extern crate libc;
#[cfg(feature = "va-list")]
extern crate va_list;

mod laux;
mod lconf;
mod lcore;
mod ldebug;

pub use laux::*;
pub use lconf::*;
pub use lcore::*;
pub use ldebug::*;

#[cfg(lua_32_bits)]
pub fn foo() {
    println!("BAR!");
}
