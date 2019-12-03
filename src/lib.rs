//! High level bindings to Lua 5.3

#![cfg_attr(rust_nightly, feature(non_exhaustive))]

extern crate libc;
pub extern crate lua_sys as sys;

use std::{error, fmt, ptr};

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
#[cfg_attr(rust_nightly, non_exhaustive)]
pub enum ErrorKind {
    Runtime,
    Syntax,
    OutOfMemory,
    MessageHandler,
    GarbageCollection,
    Io,
    #[doc(hidden)]
    #[cfg(not(rust_nightly))]
    _NonExhaustive,
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

impl error::Error for Error {
    fn description(&self) -> &str {
        #[allow(unreachable_patterns)]
        match self.kind {
            ErrorKind::Runtime => "runtime error",
            ErrorKind::Syntax => "syntax error",
            ErrorKind::OutOfMemory => "out of memory",
            ErrorKind::MessageHandler => "error while running the message handler",
            ErrorKind::GarbageCollection => "error while running a __gc metamethod",
            ErrorKind::Io => "IO error",
            _ => "unknown error",
        }
    }
    fn cause(&self) -> Option<&dyn error::Error> {
        None
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(error::Error::description(self))?;
        match &self.msg {
            Some(msg) => write!(f, ": {}", msg),
            None => Ok(()),
        }
    }
}

/// The Lua result type
pub type LuaResult<T> = Result<T, Error>;
