extern crate pollua;

use pollua::sys;
use pollua::thread::LoadingMode;
use pollua::LuaResult;
use pollua::Thread;
use std::io;
use std::io::prelude::*;

fn init_thread(thread: &mut Thread) {
    unsafe { sys::luaL_openlibs(thread.as_raw().as_ptr()) };
}

fn run_line(thread: &mut Thread, line: &str) {
    if validate(thread.load_bytes(line, Some("<stdin>"), LoadingMode::Text)) {
        let code = unsafe { sys::lua_pcall(thread.as_raw().as_ptr(), 0, sys::LUA_MULTRET, 0) };
        validate(thread.get_error(code));
    }
}

fn validate(res: LuaResult<()>) -> bool {
    match res {
        Ok(()) => true,
        Err(e) => {
            println!("\u{001b}[31;1m{}\u{001b}[0m", e);
            false
        }
    }
}

fn main() {
    let mut thread = Thread::new().expect("unable to crate Lua thread");
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
