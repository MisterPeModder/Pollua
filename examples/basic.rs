extern crate lua_sys;
extern crate pollua;

use lua_sys::*;

fn main() {
    unsafe {
        let state = pollua::State::default();
        luaL_openlibs(state.as_mut_ptr());
        println!("Lua version: {}", *lua_version(state.as_mut_ptr()));
    }
}
