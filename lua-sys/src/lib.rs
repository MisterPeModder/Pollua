//! lua-sys - Raw ffi bindings for Lua 5.3

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]
#![cfg_attr(rust_nightly, allow(clippy::missing_safety_doc))]

extern crate libc;
#[cfg(feature = "va-list")]
extern crate va_list;

mod conf;
mod debug;
mod lcore;

pub use conf::*;
pub use debug::*;
pub use lcore::*;

#[cfg(lua_32_bits)]
pub fn foo() {
    println!("BAR!");
}
