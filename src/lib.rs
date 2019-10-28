#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::not_unsafe_ptr_arg_deref)]

extern crate libc;
extern crate lua_sys;

use core::ptr;
use libc::*;
use lua_sys::*;

#[cfg(feature = "export-sys")]
pub mod sys {
    pub use lua_sys::*;
}

#[repr(transparent)]
pub struct State {
    inner: *mut lua_State,
}

impl State {
    /// Creates a state from an allocator and user data.
    #[inline]
    pub fn new(allocator: lua_Alloc, user_data: *mut c_void) -> State {
        unsafe {
            State {
                inner: lua_newstate(allocator, user_data),
            }
        }
    }

    /// Returns an raw pointer to the lua state.
    #[inline]
    pub fn as_ptr(&self) -> *const lua_State {
        self.inner
    }

    /// Returns an unsafe mutable pointer to the lua state.
    #[inline]
    pub fn as_mut_ptr(&self) -> *mut lua_State {
        self.inner
    }

    /// Creates a State from a raw pointer.
    ///
    /// # Safety
    /// Behavior is undefined if `ptr` does not point to a valid `lua_State`
    #[inline]
    pub unsafe fn from_ptr(ptr: *mut lua_State) -> State {
        State { inner: ptr }
    }
}

impl Default for State {
    #[inline]
    fn default() -> State {
        unsafe {
            State {
                inner: lua_newstate(Some(lua_alloc), ptr::null_mut()),
            }
        }
    }
}

impl Drop for State {
    fn drop(&mut self) {
        unsafe {
            lua_close(self.inner);
        }
    }
}

unsafe extern "C" fn lua_alloc(
    _ud: *mut libc::c_void,
    ptr: *mut libc::c_void,
    _osize: usize,
    nsize: usize,
) -> *mut libc::c_void {
    if nsize == 0 {
        libc::free(ptr);
        ptr::null_mut()
    } else {
        libc::realloc(ptr, nsize)
    }
}
