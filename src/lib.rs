//! High level bindings to Lua 5.3
//!
//! # no_std support
//!
//! By default, `pollua` depends on `libstd`. However, it can be configured to use the unstable
//! `liballoc` API instead.
//! This configuration is currently unstable and is not guaranteed to work on all versions of Rust.

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::string::String;

extern crate libc;
pub extern crate lua_sys as sys;

use core::fmt;
use core::ptr;
#[cfg(feature = "std")]
use std::error;

/// Lua thread API.
pub mod thread;
/// Useful functions.
pub(crate) mod util;
/// WIP
pub mod value;

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

/// The Lua error type.
#[derive(Debug, Clone)]
pub struct Error {
    kind: ErrorKind,
    msg: Option<String>,
}

/// A list specifying categories of Lua errors.
/// It is used with the [`Error`] type.
///
/// [`Error`]: struct.Error.html
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ErrorKind {
    Runtime,
    Syntax,
    OutOfMemory,
    MessageHandler,
    GarbageCollection,
    Io,
}

impl Error {
    #[inline]
    fn new(kind: ErrorKind, msg: Option<String>) -> Error {
        Error { kind, msg }
    }

    /// Returns the corresponding `ErrorKind` for this error.
    #[inline]
    pub fn kind(&self) -> ErrorKind {
        self.kind
    }

    /// Returns the message associated with this error.
    #[inline]
    pub fn msg(&self) -> Option<&str> {
        self.msg.as_ref().map(|m| &**m)
    }
}

#[cfg(feature = "std")]
impl error::Error for Error {
    fn description(&self) -> &str {
        match self.kind {
            ErrorKind::Runtime => "runtime error",
            ErrorKind::Syntax => "syntax error",
            ErrorKind::OutOfMemory => "out of memory",
            ErrorKind::MessageHandler => "error while running the message handler",
            ErrorKind::GarbageCollection => "error while running a __gc metamethod",
            ErrorKind::Io => "IO error",
        }
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        None
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self.kind {
            ErrorKind::Runtime => "runtime error",
            ErrorKind::Syntax => "syntax error",
            ErrorKind::OutOfMemory => "out of memory",
            ErrorKind::MessageHandler => "error while running the message handler",
            ErrorKind::GarbageCollection => "error while running a __gc metamethod",
            ErrorKind::Io => "IO error",
        })?;
        match &self.msg {
            Some(msg) => write!(f, ": {}", msg),
            None => Ok(()),
        }
    }
}

/// The Lua result type
pub type LuaResult<T> = Result<T, Error>;
