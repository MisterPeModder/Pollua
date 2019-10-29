// Lua standard libraries

use crate::*;

// //////////////////////////////////////////// //
// Constants                                    //
// //////////////////////////////////////////// //

pub const LUA_COLIBNAME: &str = "coroutine";
pub const LUA_TABLIBNAME: &str = "table";
pub const LUA_IOLIBNAME: &str = "io";
pub const LUA_OSLIBNAME: &str = "os";
pub const LUA_STRLIBNAME: &str = "string";

#[cfg(LUA_VERSION = "5.3")]
pub const LUA_UTF8LIBNAME: &str = "utf8";

#[cfg(LUA_VERSION = "5.2")]
pub const LUA_BITLIBNAME: &str = "bit32";

pub const LUA_MATHLIBNAME: &str = "math";
pub const LUA_DBLIBNAME: &str = "db";
pub const LUA_LOADLIBNAME: &str = "package";

// //////////////////////////////////////////// //
// Functions                                    //
// //////////////////////////////////////////// //

extern "C" {
    pub fn luaL_openlibs(L: *mut lua_State);
    pub fn luaopen_base(L: *mut lua_State) -> libc::c_int;

    #[cfg(LUA_VERSION = "5.2")]
    pub fn luaopen_bit32(L: *mut lua_State) -> libc::c_int;

    #[cfg(LUA_VERSION = "5.2")]
    pub fn luaopen_coroutine(L: *mut lua_State) -> libc::c_int;

    pub fn luaopen_debug(L: *mut lua_State) -> libc::c_int;
    pub fn luaopen_io(L: *mut lua_State) -> libc::c_int;
    pub fn luaopen_math(L: *mut lua_State) -> libc::c_int;

    #[cfg(LUA_VERSION = "5.1")]
    pub fn luaopen_os(L: *mut lua_State) -> libc::c_int;

    pub fn luaopen_package(L: *mut lua_State) -> libc::c_int;
    pub fn luaopen_string(L: *mut lua_State) -> libc::c_int;
    pub fn luaopen_table(L: *mut lua_State) -> libc::c_int;

    #[cfg(LUA_VERSION = "5.3")]
    pub fn luaopen_utf8(L: *mut lua_State) -> libc::c_int;
}
