extern crate pollua;

use pollua::sys;
use pollua::State;

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
    let version = unsafe { *sys::lua_version(State::default().as_mut_ptr()) };
    println!("lua_version() output: {}", version);
    println!("===");
}
