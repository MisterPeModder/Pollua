extern crate pollua;

use pollua::sys;
use pollua::thread::LoadingMode;
use pollua::Error;
use pollua::LuaResult;
use pollua::Thread;
use std::io;
use std::io::prelude::*;

fn init_thread(thread: &mut Thread) {
    unsafe {
        sys::luaL_checkversion(thread.as_raw_mut());
        sys::luaL_openlibs(thread.as_raw_mut());
    }
}

fn run_line(thread: &mut Thread, line: &str) {
    let res = thread.load_bytes(line, Some("<stdin>"), LoadingMode::Text);

    if validate(thread, res) {
        validate(
            thread,
            Error::from_code(unsafe {
                sys::lua_pcall(thread.as_raw_mut(), 0, sys::LUA_MULTRET, 0)
            }),
        );
    }
}

fn validate(thread: &mut Thread, res: LuaResult<()>) -> bool {
    match res {
        Ok(()) => return true,
        Err(e) => print!("\u{001b}[31;1m{}", e),
    }
    unsafe {
        let top = sys::lua_gettop(thread.as_raw_mut());
        if sys::lua_isnone(thread.as_raw_mut(), top) == 0 {
            let err = sys::lua_tostring(thread.as_raw_mut(), top);
            if !err.is_null() {
                print!(": {}", std::ffi::CStr::from_ptr(err).to_string_lossy());
            }
        }
    }
    println!("\u{001b}[0m");
    false
}

fn main() {
    let mut thread = Thread::default();
    init_thread(&mut thread);
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut prev_line = String::new();

    println!("{} (Pollua/lua-sys)", sys::LUA_RELEASE);
    println!("Type \"exit\" to quit.\n");
    print!(">>> ");
    stdout.flush().unwrap();
    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        let line = format!("{}{}", prev_line, line.trim());
        if line == "exit" {
            break;
        }
        if line.ends_with('\\') {
            prev_line = line;
            prev_line.truncate(prev_line.len() - 1);
            prev_line.push(' ');
            print!("... ");
            stdout.flush().unwrap();
            continue;
        }
        prev_line.clear();
        run_line(&mut thread, &line);
        print!(">>> ");
        stdout.flush().unwrap();
    }
}
