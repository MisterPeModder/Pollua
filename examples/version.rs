extern crate pollua;

use pollua::sys;

fn main() {
    println!("===");
    println!("Version: {}", sys::LUA_VERSION);
    println!("Release: {}", sys::LUA_RELEASE);
    println!("===");
    println!("Major : {}", sys::LUA_VERSION_MAJOR);
    println!("Minor : {}", sys::LUA_VERSION_MINOR);
    println!("Patch : {}", sys::LUA_VERSION_RELEASE);
    println!("Number: {}", sys::LUA_VERSION_NUM);
    println!("===");
    println!("pollua::lua_version(): {}", pollua::lua_version());
    println!("===");
}
