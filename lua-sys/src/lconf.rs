use crate::*;

use core::mem;

// Defines values for the LUA_VERSION_* constants
mod version {
    include!(concat!(env!("OUT_DIR"), "/lua_version.rs"));
}

pub const LUA_VERSION_MAJOR: &str = version::VERSION_MAJOR;
pub const LUA_VERSION_MINOR: &str = version::VERSION_MINOR;
pub const LUA_VERSION_RELEASE: &str = version::VERSION_RELEASE;
pub const LUA_VERSION_NUM: lua_Number = version::VERSION_NUM;
pub const LUA_VERSION: &str = version::VERSION;
pub const LUA_RELEASE: &str = version::RELEASE;

// Integer types
cfg_if::cfg_if! {
    if #[cfg(LUA_FLOAT_TYPE="LUA_FLOAT_FLOAT")] {
        pub type lua_Number = libc::c_float;

        pub const LUA_NUMBER_FRMLEN: &str = "";
        pub const LUA_NUMBER_FMT: &str = "%.7g";
    } else if #[cfg(LUA_FLOAT_TYPE="LUA_FLOAT_LONGDOUBLE")] {
        compile_error!("LUA_FLOAT_LONGDOUBLE is not supported");
    } else if #[cfg(LUA_FLOAT_TYPE="LUA_FLOAT_DOUBLE")] {
        pub type lua_Number = libc::c_double;

        pub const LUA_NUMBER_FRMLEN: &str = "";
        pub const LUA_NUMBER_FMT: &str = "%.14g";
    } else {
        compile_error!("Lua numeric float type not defined");
    }
}

// Integer types
cfg_if::cfg_if! {
    if #[cfg(LUA_INT_TYPE="LUA_INT_INT")] {
        pub type lua_Integer = libc::c_int;
        pub type lua_Unsigned = libc::c_uint;
        pub const LUA_INTEGER_FRMLEN: &str = "";

        pub const LUA_INTEGER_FMT: &str = "%d";
    } else if #[cfg(LUA_INT_TYPE="LUA_INT_LONG")] {
        pub type lua_Integer = libc::c_long;
        pub type lua_Unsigned = libc::c_ulong;
        pub const LUA_INTEGER_FRMLEN: &str = "l";

        pub const LUA_INTEGER_FMT: &str = "%ld";
    } else if #[cfg(LUA_INT_TYPE="LUA_INT_LONGLONG")] {
        pub type lua_Integer = libc::c_longlong;
        pub type lua_Unsigned = libc::c_ulonglong;
        pub const LUA_INTEGER_FRMLEN: &str = "ll";

        pub const LUA_INTEGER_FMT: &str = "%ld";
    } else {
        compile_error!("Lua numeric integer type not defined");
    }
}

// Lua 5.2 Compatibility
cfg_if::cfg_if! {
    if #[cfg(any(LUA_COMPAT_5_2, feature = "lua-compat"))] {
        pub const LUA_COMPAT_MATHLIB: bool = true;
        pub const LUA_COMPAT_BITLIB: bool = true;
        pub const LUA_COMPAT_IPAIRS: bool = true;
        pub const LUA_COMPAT_APIINTCASTS: bool = true;
    } else {
        pub const LUA_COMPAT_MATHLIB: bool = false;
        pub const LUA_COMPAT_BITLIB: bool = false;
        pub const LUA_COMPAT_IPAIRS: bool = false;
        pub const LUA_COMPAT_APIINTCASTS: bool = false;
    }
}

// Lua 5.1 Compatibility
cfg_if::cfg_if! {
    if #[cfg(any(LUA_COMPAT_5_1, feature = "lua-compat"))] {
        // Already defined if Lua 5.2 compat is enabled
        #[cfg(all(not(LUA_COMPAT_5_2), not(feature = "lua-compat")))]
        pub const LUA_COMPAT_MATHLIB: bool = true;
        #[cfg(all(not(LUA_COMPAT_5_2), not(feature = "lua-compat")))]
        pub const LUA_COMPAT_APIINTCASTS: bool = true;

        pub const LUA_COMPAT_UNPACK: bool = true;
        pub const LUA_COMPAT_LOADERS: bool = true;
        pub const LUA_COMPAT_LOG10: bool = true;
        pub const LUA_COMPAT_LOADSTRING: bool = true;
        pub const LUA_COMPAT_MAXN: bool = true;
        pub const LUA_COMPAT_MODULE: bool = true;
    } else {
        pub const LUA_COMPAT_UNPACK: bool = false;
        pub const LUA_COMPAT_LOADERS: bool = false;
        pub const LUA_COMPAT_LOG10: bool = false;
        pub const LUA_COMPAT_LOADSTRING: bool = false;
        pub const LUA_COMPAT_MAXN: bool = false;
        pub const LUA_COMPAT_MODULE: bool = false;
    }
}

// KContest
cfg_if::cfg_if! {
    if #[cfg(LUA_USE_C89)] {
        pub type lua_KContext = libc::ptrdiff_t;
    } else {
        pub type lua_KContext = libc::intptr_t;
    }
}

// luaxlib buffer size
cfg_if::cfg_if! {
    if #[cfg(LUA_FLOAT_TYPE="LUA_FLOAT_LONGDOUBLE")] {
        pub const LUAL_BUFFERSIZE: usize = 8192;
    } else {
        pub const LUAL_BUFFERSIZE: usize =
            0x80 * mem::size_of::<*const libc::c_void>() * mem::size_of::<lua_Integer>();
    }
}
