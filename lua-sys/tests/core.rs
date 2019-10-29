extern crate lua_sys;
mod common;

use common::*;
use lua_sys::*;
use std::ffi::CStr;

#[test]
fn test_check_version() {
    run_thread(|l| unsafe {
        assert_eq!(503.0, *lua_version(l));
    });
}

#[test]
fn test_arith() {
    run_thread(|l| unsafe {
        lua_pushinteger(l, 1);
        lua_pushinteger(l, 2);
        lua_arith(l, LUA_OPADD);
        assert_eq!(lua_tointeger(l, 1), 3);

        lua_pushinteger(l, 10);
        lua_arith(l, LUA_OPSUB);
        assert_eq!(lua_tointeger(l, 1), -7);
    });
}

#[test]
fn test_call_cfunction() {
    unsafe extern "C" fn sum(l: *mut lua_State) -> libc::c_int {
        let mut sum: lua_Number = 0.0;
        let nargs = lua_gettop(l);
        for i in 1..=nargs {
            if lua_isnumber(l, i) == 0 {
                lua_pushliteral(l, cstr(b"argument must be a number\0"));
                lua_error(l);
                unreachable!();
            }
            sum += lua_tonumber(l, i);
        }
        lua_pushnumber(l, sum); // result 1: the sum
        lua_pushinteger(l, nargs as lua_Integer); // result 2: number of args
        return 2;
    }

    run_thread(|l| unsafe {
        let fname = cstr(b"sum\0");
        lua_register(l, fname, Some(sum));

        // sum() -- 0, 0
        lua_getglobal(l, fname);
        lua_call(l, 0, 2);
        assert_eq!(lua_tointeger(l, 1), 0);
        assert_eq!(lua_tointeger(l, 2), 0);
        lua_pop(l, 2);

        // sum(24) -- 24, 1
        lua_getglobal(l, fname);
        lua_pushinteger(l, 24);
        lua_call(l, 1, 2);
        assert_eq!(lua_tointeger(l, 1), 24);
        assert_eq!(lua_tointeger(l, 2), 1);
        lua_pop(l, 2);

        // sum(24, 68, 92, 0) -- 24, 4
        lua_getglobal(l, fname);
        lua_pushinteger(l, 24);
        lua_pushinteger(l, 68);
        lua_pushinteger(l, 92);
        lua_pushinteger(l, 0);
        lua_call(l, 4, 2);
        assert_eq!(lua_tointeger(l, 1), 184);
        assert_eq!(lua_tointeger(l, 2), 4);
        lua_pop(l, 2);

        // sum("2", "text") -- error
        lua_getglobal(l, fname);
        lua_pushliteral(l, cstr(b"2\0"));
        lua_pushliteral(l, cstr(b"text\0"));
        assert_eq!(lua_pcall(l, 2, 2, 0), LUA_ERRRUN);
        assert_eq!(
            CStr::from_ptr(lua_tostring(l, 1)),
            CStr::from_bytes_with_nul(b"argument must be a number\0").unwrap()
        );
        lua_pop(l, 2);
    });
}
