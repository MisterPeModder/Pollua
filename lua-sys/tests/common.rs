use lua_sys::*;
use std::ffi::CStr;

#[inline]
pub fn cstr(literal: &'static [u8]) -> *const libc::c_char {
    CStr::from_bytes_with_nul(literal)
        .expect("invalid c string")
        .as_ptr()
}

pub fn run_thread<F: Fn(*mut lua_State)>(func: F) {
    unsafe extern "C" fn at_panic(l: *mut lua_State) -> libc::c_int {
        panic!(
            "Lua panic: {}",
            CStr::from_ptr(lua_tostring(l, -1)).to_string_lossy()
        );
    }

    unsafe {
        let l = luaL_newstate();
        lua_atpanic(l, Some(at_panic));
        func(l);
        lua_close(l);
    }
}
