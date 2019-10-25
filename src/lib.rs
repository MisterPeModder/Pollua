/*
 * Copyright (c) 2019 Yanis Guaye
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http: //www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#![allow(clippy::not_unsafe_ptr_arg_deref)]

extern crate lua_sys;

use lua_sys::*;
use std::ffi::c_void;

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
                inner: luaL_newstate(),
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
