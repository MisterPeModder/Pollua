extern crate lua_sys;

use lua_sys::*;
use std::mem::MaybeUninit;

#[test]
fn test_buffers() {
    unsafe {
        let state = luaL_newstate();
        let mut buff_val = MaybeUninit::<luaL_Buffer>::uninit();
        let buff = buff_val.as_mut_ptr();

        luaL_buffinit(state, buff);

        let ptr = luaL_prepbuffer(buff);
        ptr.copy_from("Hello World".as_ptr() as _, 11);
        luaL_addsize(buff, 11);
        luaL_addchar(buff, b'!' as _);
        luaL_pushresult(buff);

        let mut result_len = 0usize;
        let result = std::str::from_utf8(std::slice::from_raw_parts(
            lua_tolstring(state, -1, &mut result_len as *mut _) as *const _,
            result_len,
        ))
        .unwrap();

        assert_eq!(result, "Hello World!");
        lua_close(state);
    }
}
