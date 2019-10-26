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

use crate::*;

//////////////////////////////////////////////////
// Lua types                                    //
//////////////////////////////////////////////////

#[repr(C)]
pub struct lua_Debug {
    event: libc::c_int,
    name: *const libc::c_char,
    namewhat: *const libc::c_char,
    what: *const libc::c_char,
    source: *const libc::c_char,
    currentline: libc::c_int,
    linedefined: libc::c_int,
    lastlinedefined: libc::c_int,
    nups: libc::c_uchar,
    nparams: libc::c_uchar,
    isvararg: libc::c_char,
    istailcall: libc::c_char,
    short_src: [libc::c_char; LUA_IDSIZE],
    /* private part */
    _private: private::lua_Debug,
}

pub type lua_Hook = Option<unsafe extern "C" fn(L: *mut lua_State, ar: *mut lua_Debug)>;

#[allow(unused)]
mod private {
    use super::*;

    #[repr(C)]
    pub struct lua_Debug {
        i_ci: *mut CallInfo,
    }

    #[repr(C)]
    struct CallInfo {
        _private: [u8; 0],
    }
}

//////////////////////////////////////////////////
// Lua Functions                                //
//////////////////////////////////////////////////

extern "C" {
    pub fn lua_gethook(L: *mut lua_State) -> lua_Hook;
    pub fn lua_gethookcount(L: *mut lua_State) -> libc::c_int;
    pub fn lua_gethookmask(L: *mut lua_State) -> libc::c_int;
    pub fn lua_getinfo(
        L: *mut lua_State,
        what: *const libc::c_char,
        ar: *mut lua_Debug,
    ) -> libc::c_int;
    pub fn lua_getlocal(
        L: *mut lua_State,
        ar: *const lua_Debug,
        n: libc::c_int,
    ) -> *const libc::c_char;
    pub fn lua_getstack(L: *mut lua_State, level: libc::c_int, ar: *mut lua_Debug) -> libc::c_int;
    pub fn lua_getupvalue(
        L: *mut lua_State,
        funcindex: libc::c_int,
        n: libc::c_int,
    ) -> *const libc::c_char;
    pub fn lua_sethook(L: *mut lua_State, func: lua_Hook, mask: libc::c_int, count: libc::c_int);
    pub fn lua_setlocal(
        L: *mut lua_State,
        ar: *const lua_Debug,
        n: libc::c_int,
    ) -> *const libc::c_char;
    pub fn lua_setupvalue(
        L: *mut lua_State,
        funcindex: libc::c_int,
        n: libc::c_int,
    ) -> *const libc::c_char;
    pub fn lua_upvalueid(L: *mut lua_State, fidx: libc::c_int, n: libc::c_int)
        -> *mut libc::c_void;
    pub fn lua_upvaluejoin(
        L: *mut lua_State,
        fidx1: libc::c_int,
        n1: libc::c_int,
        fidx2: libc::c_int,
        n2: libc::c_int,
    );
}
