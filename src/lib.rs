//! High level bindings to Lua 5.3
//!
//! # no_std support
//!
//! By default, `pollua` depends on `libstd`. However, it can be configured to use the unstable
//! `liballoc` API instead.
//! This configuration is currently unstable and is not guaranteed to work on all versions of Rust.

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), feature(alloc))]

#[cfg(not(feature = "std"))]
#[macro_use]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

extern crate libc;
pub extern crate lua_sys as sys;

use core::fmt;
use core::ptr;
#[cfg(feature = "std")]
use std::error;

pub mod thread;

pub use thread::Thread;

/// Returns the version number stored in the Lua core.
///
/// # Examples
/// ```
/// println!("Lua version number: {}", pollua::lua_version());
/// ```
#[inline]
pub fn lua_version() -> sys::lua_Number {
    unsafe { *sys::lua_version(ptr::null_mut()) }
}

/// The Lua error type
#[derive(Debug)]
pub enum Error {
    Runtime,
    Syntax,
    OutOfMemory,
    MessageHandler,
    GarbageCollection,
    Io,
}

impl Error {
    pub fn from_code(code: i32) -> LuaResult<()> {
        match code {
            sys::LUA_ERRRUN => Err(Error::Runtime),
            sys::LUA_ERRSYNTAX => Err(Error::Syntax),
            sys::LUA_ERRMEM => Err(Error::OutOfMemory),
            sys::LUA_ERRERR => Err(Error::MessageHandler),
            sys::LUA_ERRGCMM => Err(Error::GarbageCollection),
            sys::LUA_ERRFILE => Err(Error::Io),
            _ => Ok(()),
        }
    }

    pub fn get_code(res: LuaResult<()>) -> i32 {
        (match res {
            Ok(()) => sys::LUA_OK,
            Err(Error::Runtime) => sys::LUA_ERRRUN,
            Err(Error::Syntax) => sys::LUA_ERRSYNTAX,
            Err(Error::OutOfMemory) => sys::LUA_ERRMEM,
            Err(Error::MessageHandler) => sys::LUA_ERRERR,
            Err(Error::GarbageCollection) => sys::LUA_ERRGCMM,
            Err(Error::Io) => sys::LUA_ERRFILE,
        }) as i32
    }
}

#[cfg(feature = "std")]
impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Runtime => "runtime error",
            Error::Syntax => "syntax error",
            Error::OutOfMemory => "out of memory",
            Error::MessageHandler => "error while running the message handler",
            Error::GarbageCollection => "error while running a __gc metamethod",
            Error::Io => "IO error",
        }
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        None
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Runtime => write!(f, "runtime error"),
            Error::Syntax => write!(f, "syntax error"),
            Error::OutOfMemory => write!(f, "memory allocation error"),
            Error::MessageHandler => write!(f, "error while running the message handler"),
            Error::GarbageCollection => write!(f, "error while running a __gc metamethod"),
            Error::Io => write!(f, "IO error"),
        }
    }
}

/// The Lua result type
pub type LuaResult<T> = Result<T, Error>;

// Returns a pointer to s is s is a valid c string,
// otherwise copies to s to buf, removes nul bytes and adds the final nul byte.
fn cstr_buf<S: AsRef<[u8]>>(s: Option<S>, buf: &mut Vec<u8>) -> *mut libc::c_char {
    match s {
        Some(s) => {
            let s = s.as_ref();
            let nulb =
                unsafe { libc::memchr(s.as_ptr() as *const libc::c_void, 0, s.len()) as usize };
            // check if the only nul byte is at the end
            (if nulb as usize == s.as_ptr() as usize + s.len() - 1 {
                s.as_ptr()
            } else {
                buf.clear();
                buf.extend(s.iter().filter(|&&b| b != 0).chain(core::iter::once(&0u8)));
                buf.as_mut_ptr()
            }) as *mut libc::c_char
        }
        None => ptr::null_mut(),
    }
}

#[inline]
unsafe fn cstr_unchecked<S: AsRef<[u8]>>(s: Option<S>) -> *const libc::c_char {
    match s {
        Some(s) => s.as_ref().as_ptr() as *const libc::c_char,
        None => ptr::null(),
    }
}
